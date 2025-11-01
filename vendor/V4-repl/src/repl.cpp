#include "repl.hpp"

#ifndef _WIN32
#include <signal.h>
#include <unistd.h>
#endif

#include <cstdio>
#include <cstdlib>
#include <cstring>

#ifndef _WIN32
// Unix: use linenoise for line editing
extern "C" {
#include "linenoise.h"
}
#else
// Windows: use simple line input with std::string
#include <iostream>
#include <string>
#include <vector>
#endif

#ifdef WITH_FILESYSTEM
#ifdef _WIN32
#include <direct.h>  // for _mkdir
#include <io.h>      // for _access
#define ACCESS _access
#define MKDIR(path) _mkdir(path)
#else
#include <sys/stat.h>
#include <unistd.h>
#define ACCESS access
#define MKDIR(path) mkdir(path, 0755)
#endif
#endif

static const char* PROMPT = "v4> ";
static const int MAX_HISTORY = 1000;

#ifndef _WIN32
// Unix: Global interrupt flag for Ctrl+C handling
static volatile sig_atomic_t g_interrupted = 0;

// Signal handler for Ctrl+C
static void sigint_handler(int sig) {
  (void) sig;
  g_interrupted = 1;

  // Safe message output (signal-safe)
  const char msg[] = "\n^C\n";
  ssize_t written = write(STDERR_FILENO, msg, sizeof(msg) - 1);
  (void) written;  // Suppress unused warning
}
#else
// Windows: Dummy interrupt flag (Ctrl+C not implemented)
static volatile int g_interrupted = 0;
#endif

#ifdef _WIN32
// Windows: Simple line history vector
static std::vector<std::string> g_history;
static size_t g_history_index = 0;

static void add_history(const std::string& line) {
  if (!line.empty() && (g_history.empty() || g_history.back() != line)) {
    g_history.push_back(line);
    if (g_history.size() > MAX_HISTORY) {
      g_history.erase(g_history.begin());
    }
  }
  g_history_index = g_history.size();
}

static void load_history(const char* path) {
  FILE* f = fopen(path, "r");
  if (!f)
    return;

  char line[1024];
  while (fgets(line, sizeof(line), f)) {
    // Remove trailing newline
    size_t len = strlen(line);
    if (len > 0 && line[len - 1] == '\n')
      line[len - 1] = '\0';
    if (line[0])
      g_history.push_back(line);
  }
  fclose(f);
  g_history_index = g_history.size();
}

static void save_history(const char* path) {
  FILE* f = fopen(path, "w");
  if (!f)
    return;

  for (const auto& line : g_history) {
    fprintf(f, "%s\n", line.c_str());
  }
  fclose(f);
}

// Simple line reading for Windows
static char* read_line_simple(const char* prompt) {
  printf("%s", prompt);
  fflush(stdout);

  std::string line;
  if (!std::getline(std::cin, line)) {
    return nullptr;  // EOF (Ctrl+Z on Windows)
  }

  char* result = (char*) malloc(line.length() + 1);
  if (result) {
    strcpy(result, line.c_str());
  }
  return result;
}
#endif

Repl::Repl()
    : vm_(nullptr),
      compiler_ctx_(nullptr),
      meta_cmds_(nullptr, nullptr),
      word_bufs_(nullptr),
      word_buf_count_(0),
      word_buf_capacity_(0),
      paste_mode_(false),
      paste_buffer_(nullptr),
      paste_buffer_size_(0),
      paste_buffer_capacity_(0) {
  // Initialize VM memory to zero
  memset(vm_memory_, 0, sizeof(vm_memory_));

  // Create VM configuration
  VmConfig cfg = {0};  // Zero-initialize to avoid uninitialized fields
  cfg.mem = vm_memory_;
  cfg.mem_size = sizeof(vm_memory_);
  cfg.mmio = nullptr;
  cfg.mmio_count = 0;
  cfg.arena = nullptr;  // Explicitly set arena to NULL (use malloc for word names)

  // Create VM instance
  vm_ = vm_create(&cfg);
  if (!vm_) {
    fprintf(stderr, "Failed to create VM\n");
    exit(1);
  }

  // Create compiler context
  compiler_ctx_ = v4front_context_create();
  if (!compiler_ctx_) {
    fprintf(stderr, "Failed to create compiler context\n");
    vm_destroy(vm_);
    exit(1);
  }

  // Initialize meta-commands handler
  meta_cmds_ = MetaCommands(vm_, compiler_ctx_);

#ifndef _WIN32
  // Set up Ctrl+C signal handler (Unix only)
  struct sigaction sa;
  sa.sa_handler = sigint_handler;
  sigemptyset(&sa.sa_mask);
  sa.sa_flags = 0;
  sigaction(SIGINT, &sa, nullptr);
#endif

#ifdef WITH_FILESYSTEM
  init_history();
#endif
}

Repl::~Repl() {
#ifdef WITH_FILESYSTEM
  save_history();
#endif

  if (compiler_ctx_) {
    v4front_context_destroy(compiler_ctx_);
    compiler_ctx_ = nullptr;
  }

  if (vm_) {
    vm_destroy(vm_);
    vm_ = nullptr;
  }

  // Free all tracked word definition buffers
  for (int i = 0; i < word_buf_count_; ++i) {
    v4front_free(&word_bufs_[i]);
  }
  free(word_bufs_);

  // Free PASTE buffer
  free(paste_buffer_);
}

#ifdef WITH_FILESYSTEM
void Repl::init_history() {
#ifdef _WIN32
  // Windows: Get home directory from USERPROFILE
  const char* home = getenv("USERPROFILE");
  if (!home) {
    home = ".";
  }
  // Construct history file path
  snprintf(history_path_, sizeof(history_path_), "%s\\.v4_history", home);
  // Load existing history
  ::load_history(history_path_);
#else
  // Unix: Get home directory from HOME
  const char* home = getenv("HOME");
  if (!home) {
    home = ".";
  }
  // Construct history file path
  snprintf(history_path_, sizeof(history_path_), "%s/.v4_history", home);
  // Set max history
  linenoiseHistorySetMaxLen(MAX_HISTORY);
  // Load existing history
  linenoiseHistoryLoad(history_path_);
#endif
}

void Repl::save_history() {
#ifdef _WIN32
  ::save_history(history_path_);
#else
  linenoiseHistorySave(history_path_);
#endif
}
#endif

void Repl::print_stack() {
  int depth = vm_ds_depth_public(vm_);

  if (depth == 0) {
    printf(" ok\n");
    return;
  }

  printf(" ok [%d]:", depth);

  // Print stack from bottom to top
  for (int i = depth - 1; i >= 0; --i) {
    v4_i32 val = vm_ds_peek_public(vm_, i);
    printf(" %d", val);
  }

  printf("\n");
}

void Repl::print_error(const char* msg, int code) {
  if (code != 0) {
    fprintf(stderr, "Error [%d]: %s\n", code, msg);
  } else {
    fprintf(stderr, "Error: %s\n", msg);
  }
}

bool Repl::is_paste_marker(const char* line) {
  // Skip leading whitespace
  while (*line == ' ' || *line == '\t') {
    line++;
  }

  if (strncmp(line, "<<<", 3) == 0) {
    // Check that it's just <<< with optional trailing whitespace
    const char* p = line + 3;
    while (*p == ' ' || *p == '\t' || *p == '\n')
      p++;
    return (*p == '\0');
  }

  if (strncmp(line, ">>>", 3) == 0) {
    // Check that it's just >>> with optional trailing whitespace
    const char* p = line + 3;
    while (*p == ' ' || *p == '\t' || *p == '\n')
      p++;
    return (*p == '\0');
  }

  return false;
}

void Repl::enter_paste_mode() {
  paste_mode_ = true;
  paste_buffer_size_ = 0;
  if (paste_buffer_) {
    paste_buffer_[0] = '\0';
  }
  printf("Entering PASTE mode. Type '>>>' to compile and execute.\n");
}

void Repl::exit_paste_mode() {
  paste_mode_ = false;

  if (paste_buffer_size_ == 0 || !paste_buffer_) {
    printf("(empty PASTE buffer)\n");
    return;
  }

  // Compile and execute the buffered code
  int result = eval_line(paste_buffer_);

  if (result == 0) {
    print_stack();
  }

  // Clear buffer
  paste_buffer_size_ = 0;
  if (paste_buffer_) {
    paste_buffer_[0] = '\0';
  }
}

const char* Repl::get_prompt() const {
  return paste_mode_ ? "... " : "v4> ";
}

int Repl::eval_line(const char* line) {
  // Clear interrupt flag at the start of evaluation
  g_interrupted = 0;

  // Check for PASTE mode markers
  if (is_paste_marker(line)) {
    // Skip whitespace
    while (*line == ' ' || *line == '\t')
      line++;

    if (strncmp(line, "<<<", 3) == 0) {
      if (paste_mode_) {
        printf("Already in PASTE mode\n");
      } else {
        enter_paste_mode();
      }
      return 0;
    } else {  // >>>
      if (!paste_mode_) {
        printf("Not in PASTE mode\n");
      } else {
        exit_paste_mode();
      }
      return 0;
    }
  }

  // If in PASTE mode, accumulate lines
  if (paste_mode_) {
    int line_len = strlen(line);
    int needed = paste_buffer_size_ + line_len + 2;  // +2 for '\n' and '\0'

    // Grow buffer if needed
    if (needed > paste_buffer_capacity_) {
      int new_cap = (paste_buffer_capacity_ == 0) ? 1024 : (paste_buffer_capacity_ * 2);
      while (new_cap < needed) {
        new_cap *= 2;
      }

      char* new_buf = (char*) realloc(paste_buffer_, new_cap);
      if (!new_buf) {
        fprintf(stderr, "Out of memory in PASTE mode\n");
        paste_mode_ = false;
        return -1;
      }

      paste_buffer_ = new_buf;
      paste_buffer_capacity_ = new_cap;
    }

    // Append line with newline
    strcpy(paste_buffer_ + paste_buffer_size_, line);
    paste_buffer_size_ += line_len;
    paste_buffer_[paste_buffer_size_++] = '\n';
    paste_buffer_[paste_buffer_size_] = '\0';

    return 0;  // Continue accumulating
  }

  // Check for exit command
  if (strcmp(line, "bye") == 0 || strcmp(line, "quit") == 0) {
    return 1;  // Signal to exit
  }

  // Skip empty lines
  if (line[0] == '\0' || line[0] == '\n') {
    return 0;
  }

  // Check for meta-commands
  if (meta_cmds_.execute(line)) {
    return 0;  // Meta-command executed
  }

  // Check for interrupt before compilation
  if (g_interrupted) {
    fprintf(stderr, "Interrupted\n");
    vm_ds_clear(vm_);
    g_interrupted = 0;
    return -1;
  }

  // Compile the input with context and detailed error information
  V4FrontBuf buf;
  memset(&buf, 0, sizeof(buf));

  V4FrontError error;
  v4front_err err = v4front_compile_with_context_ex(compiler_ctx_, line, &buf, &error);

  if (err != 0) {
    // Format and display detailed error message
    char formatted_error[1024];
    v4front_format_error(&error, line, formatted_error, sizeof(formatted_error));
    fprintf(stderr, "%s\n", formatted_error);
    return -1;
  }

  // Register any defined words to VM and compiler context
  for (int i = 0; i < buf.word_count; ++i) {
    V4FrontWord* word = &buf.words[i];

    // Register to VM
    int wid = vm_register_word(vm_, word->name, word->code, static_cast<int>(word->code_len));

    if (wid < 0) {
      print_error("Failed to register word definition", wid);
      // Error during word registration - buffer not yet saved, must free
      v4front_free(&buf);
      return -1;
    }

    // Register to compiler context
    v4front_err ctx_err = v4front_context_register_word(compiler_ctx_, word->name, wid);
    if (ctx_err != 0) {
      print_error("Failed to register word to compiler context", ctx_err);
      // Error during word registration - buffer not yet saved, must free
      v4front_free(&buf);
      return -1;
    }
  }

  // If we defined any words, save the buffer (VM holds pointers to the bytecode)
  bool has_word_defs = (buf.word_count > 0);
  if (has_word_defs) {
    // Grow word_bufs_ array if needed
    if (word_buf_count_ >= word_buf_capacity_) {
      int new_cap = (word_buf_capacity_ == 0) ? 16 : (word_buf_capacity_ * 2);
      V4FrontBuf* new_bufs = (V4FrontBuf*) realloc(word_bufs_, new_cap * sizeof(V4FrontBuf));
      if (!new_bufs) {
        print_error("Out of memory tracking word definitions", 0);
        return -1;
      }
      word_bufs_ = new_bufs;
      word_buf_capacity_ = new_cap;
    }
    // Save this buffer (will be freed in destructor)
    word_bufs_[word_buf_count_++] = buf;
  }

  // Register and execute main code
  if (buf.data && buf.size > 0) {
    int wid = vm_register_word(vm_, nullptr, buf.data, static_cast<int>(buf.size));

    if (wid < 0) {
      print_error("Failed to register word", wid);
      if (!has_word_defs) {
        v4front_free(&buf);
      }
      return -1;
    }

    struct Word* entry = vm_get_word(vm_, wid);
    if (!entry) {
      print_error("Failed to get word entry", 0);
      if (!has_word_defs) {
        v4front_free(&buf);
      }
      return -1;
    }

    v4_err exec_err = vm_exec(vm_, entry);

    // Check for interrupt after execution
    if (g_interrupted) {
      fprintf(stderr, "Execution interrupted\n");
      vm_ds_clear(vm_);
      g_interrupted = 0;
      if (!has_word_defs) {
        v4front_free(&buf);
      }
      return -1;
    }

    if (exec_err != 0) {
      print_error("Execution failed", exec_err);
      if (!has_word_defs) {
        v4front_free(&buf);
      }
      return -1;
    }
  }

  // Free compiler output if no word definitions
  // (word definitions are kept alive and freed in destructor)
  if (!has_word_defs) {
    v4front_free(&buf);
  }

  return 0;  // Success
}

int Repl::run() {
  printf("V4 REPL v0.4.0\n");
#ifdef _WIN32
  printf("Type 'bye' or press Ctrl+Z to exit\n");
#else
  printf("Type 'bye' or press Ctrl+D to exit\n");
#endif
  printf("Type '.help' for help\n");
  printf("Type '<<<' to enter PASTE mode\n\n");

  while (true) {
#ifndef _WIN32
    // Clear interrupt flag before reading input (Unix only)
    g_interrupted = 0;

    char* line = linenoise(get_prompt());

    // Ctrl+D pressed
    if (!line) {
      printf("\nGoodbye!\n");
      break;
    }

    // Check if interrupted during input
    if (g_interrupted) {
      linenoiseFree(line);
      // If in PASTE mode, exit it
      if (paste_mode_) {
        paste_mode_ = false;
        paste_buffer_size_ = 0;
        printf("PASTE mode interrupted\n");
      }
      g_interrupted = 0;
      continue;
    }
#else
    // Windows: Simple line reading
    char* line = read_line_simple(get_prompt());

    // EOF (Ctrl+Z) pressed
    if (!line) {
      printf("\nGoodbye!\n");
      break;
    }
#endif

    // Evaluate the line
    int result = eval_line(line);

    if (result == 1) {
      // User requested exit
      printf("Goodbye!\n");
#ifndef _WIN32
      linenoiseFree(line);
#else
      free(line);
#endif
      break;
    } else if (result == 0) {
      // Success - print stack
      print_stack();

#ifdef WITH_FILESYSTEM
      // Add to history if not empty
      if (line[0] != '\0') {
#ifndef _WIN32
        linenoiseHistoryAdd(line);
#else
        ::add_history(line);
#endif
      }
#endif
    }
    // result == -1 means error, already printed

#ifndef _WIN32
    linenoiseFree(line);
#else
    free(line);
#endif
  }

  return 0;
}
