#include "v4repl/repl.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* Version: 0.4.0 */
#define V4_REPL_VERSION 0x000400

/* Default configuration */
#define DEFAULT_LINE_BUFFER_SIZE 512
#define DEFAULT_ERROR_BUFFER_SIZE 512
#define WORD_BUF_INITIAL_CAPACITY 16

/**
 * @brief Internal REPL context structure
 */
struct V4ReplContext {
  struct Vm* vm;             /* VM instance (borrowed reference) */
  V4FrontContext* front_ctx; /* Compiler context (borrowed reference) */

  /* Line buffer */
  char* line_buf;
  size_t line_buf_size;

  /* Error message buffer */
  char* error_buf;
  size_t error_buf_size;

  /* Word definition buffers (must not be freed while VM is alive) */
  V4FrontBuf* word_bufs;
  int word_buf_count;
  int word_buf_capacity;
};

/* ------------------------------------------------------------------------- */
/* Lifecycle                                                                 */
/* ------------------------------------------------------------------------- */

V4ReplContext* v4_repl_create(const V4ReplConfig* config) {
  if (!config || !config->vm || !config->front_ctx) {
    return NULL;
  }

  V4ReplContext* ctx = (V4ReplContext*) calloc(1, sizeof(V4ReplContext));
  if (!ctx) {
    return NULL;
  }

  /* Store VM and compiler context references */
  ctx->vm = config->vm;
  ctx->front_ctx = config->front_ctx;

  /* Allocate line buffer */
  ctx->line_buf_size =
      (config->line_buffer_size > 0) ? config->line_buffer_size : DEFAULT_LINE_BUFFER_SIZE;
  ctx->line_buf = (char*) malloc(ctx->line_buf_size);
  if (!ctx->line_buf) {
    free(ctx);
    return NULL;
  }

  /* Allocate error buffer */
  ctx->error_buf_size = DEFAULT_ERROR_BUFFER_SIZE;
  ctx->error_buf = (char*) malloc(ctx->error_buf_size);
  if (!ctx->error_buf) {
    free(ctx->line_buf);
    free(ctx);
    return NULL;
  }
  ctx->error_buf[0] = '\0';

  /* Initialize word buffer tracking */
  ctx->word_buf_capacity = WORD_BUF_INITIAL_CAPACITY;
  ctx->word_bufs = (V4FrontBuf*) calloc(ctx->word_buf_capacity, sizeof(V4FrontBuf));
  if (!ctx->word_bufs) {
    free(ctx->error_buf);
    free(ctx->line_buf);
    free(ctx);
    return NULL;
  }
  ctx->word_buf_count = 0;

  return ctx;
}

void v4_repl_destroy(V4ReplContext* ctx) {
  if (!ctx) {
    return;
  }

  /* Free all tracked word definition buffers */
  for (int i = 0; i < ctx->word_buf_count; ++i) {
    v4front_free(&ctx->word_bufs[i]);
  }
  free(ctx->word_bufs);

  /* Free buffers */
  free(ctx->error_buf);
  free(ctx->line_buf);

  /* Free context itself */
  free(ctx);
}

/* ------------------------------------------------------------------------- */
/* Core REPL operations                                                      */
/* ------------------------------------------------------------------------- */

v4_err v4_repl_process_line(V4ReplContext* ctx, const char* line) {
  if (!ctx || !line) {
    return -1;
  }

  /* Clear previous error */
  ctx->error_buf[0] = '\0';

  /* Skip empty lines */
  if (line[0] == '\0' || line[0] == '\n') {
    return 0;
  }

  /* Compile the input with context and detailed error information */
  V4FrontBuf buf;
  memset(&buf, 0, sizeof(buf));

  V4FrontError error;
  v4front_err err = v4front_compile_with_context_ex(ctx->front_ctx, line, &buf, &error);

  if (err != 0) {
    /* Format and store error message */
    v4front_format_error(&error, line, ctx->error_buf, ctx->error_buf_size);
    return err;
  }

  /* Register any defined words to VM and compiler context */
  for (int i = 0; i < buf.word_count; ++i) {
    V4FrontWord* word = &buf.words[i];

    /* Register to VM */
    int wid = vm_register_word(ctx->vm, word->name, word->code, (int) word->code_len);

    if (wid < 0) {
      snprintf(ctx->error_buf, ctx->error_buf_size, "Failed to register word '%s': error %d",
               word->name, wid);
      v4front_free(&buf);
      return wid;
    }

    /* Register to compiler context */
    v4front_err ctx_err = v4front_context_register_word(ctx->front_ctx, word->name, wid);
    if (ctx_err != 0) {
      snprintf(ctx->error_buf, ctx->error_buf_size,
               "Failed to register word '%s' to compiler: error %d", word->name, ctx_err);
      v4front_free(&buf);
      return ctx_err;
    }
  }

  /* If we defined any words, save the buffer (VM holds pointers to the bytecode) */
  int has_word_defs = (buf.word_count > 0);
  if (has_word_defs) {
    /* Grow word_bufs array if needed */
    if (ctx->word_buf_count >= ctx->word_buf_capacity) {
      int new_cap = ctx->word_buf_capacity * 2;
      V4FrontBuf* new_bufs = (V4FrontBuf*) realloc(ctx->word_bufs, new_cap * sizeof(V4FrontBuf));
      if (!new_bufs) {
        snprintf(ctx->error_buf, ctx->error_buf_size, "Out of memory tracking word definitions");
        v4front_free(&buf);
        return -1;
      }
      ctx->word_bufs = new_bufs;
      ctx->word_buf_capacity = new_cap;
    }
    /* Save this buffer (will be freed in destructor) */
    ctx->word_bufs[ctx->word_buf_count++] = buf;
  }

  /* Register and execute main code */
  if (buf.data && buf.size > 0) {
    int wid = vm_register_word(ctx->vm, NULL, buf.data, (int) buf.size);

    if (wid < 0) {
      snprintf(ctx->error_buf, ctx->error_buf_size, "Failed to register code: error %d", wid);
      if (!has_word_defs) {
        v4front_free(&buf);
      }
      return wid;
    }

    struct Word* entry = vm_get_word(ctx->vm, wid);
    if (!entry) {
      snprintf(ctx->error_buf, ctx->error_buf_size, "Failed to get word entry");
      if (!has_word_defs) {
        v4front_free(&buf);
      }
      return -1;
    }

    v4_err exec_err = vm_exec(ctx->vm, entry);

    if (exec_err != 0) {
      snprintf(ctx->error_buf, ctx->error_buf_size, "Execution failed: error %d", exec_err);
      if (!has_word_defs) {
        v4front_free(&buf);
      }
      return exec_err;
    }
  }

  /* Free compiler output if no word definitions */
  if (!has_word_defs) {
    v4front_free(&buf);
  }

  return 0;
}

void v4_repl_reset(V4ReplContext* ctx) {
  if (!ctx) {
    return;
  }

  /* Reset VM stacks */
  vm_reset_stacks(ctx->vm);

  /* Reset compiler context */
  v4front_context_reset(ctx->front_ctx);

  /* Free all word definition buffers */
  for (int i = 0; i < ctx->word_buf_count; ++i) {
    v4front_free(&ctx->word_bufs[i]);
  }
  ctx->word_buf_count = 0;
}

void v4_repl_reset_dictionary(V4ReplContext* ctx) {
  if (!ctx) {
    return;
  }

  /* Reset VM dictionary */
  vm_reset_dictionary(ctx->vm);

  /* Reset compiler context */
  v4front_context_reset(ctx->front_ctx);

  /* Free all word definition buffers */
  for (int i = 0; i < ctx->word_buf_count; ++i) {
    v4front_free(&ctx->word_bufs[i]);
  }
  ctx->word_buf_count = 0;
}

/* ------------------------------------------------------------------------- */
/* Stack display helpers                                                     */
/* ------------------------------------------------------------------------- */

int v4_repl_stack_depth(const V4ReplContext* ctx) {
  if (!ctx) {
    return 0;
  }
  return vm_ds_depth_public(ctx->vm);
}

void v4_repl_print_stack(const V4ReplContext* ctx) {
  if (!ctx) {
    return;
  }

  int depth = vm_ds_depth_public(ctx->vm);

  if (depth == 0) {
    printf(" ok\n");
    return;
  }

  printf(" ok [%d]:", depth);

  /* Print stack from bottom to top */
  for (int i = depth - 1; i >= 0; --i) {
    v4_i32 val = vm_ds_peek_public(ctx->vm, i);
    printf(" %d", val);
  }

  printf("\n");
}

/* ------------------------------------------------------------------------- */
/* Error handling helpers                                                    */
/* ------------------------------------------------------------------------- */

const char* v4_repl_get_error(const V4ReplContext* ctx) {
  if (!ctx) {
    return NULL;
  }

  return (ctx->error_buf[0] != '\0') ? ctx->error_buf : NULL;
}

/* ------------------------------------------------------------------------- */
/* Version information                                                       */
/* ------------------------------------------------------------------------- */

int v4_repl_version(void) {
  return V4_REPL_VERSION;
}
