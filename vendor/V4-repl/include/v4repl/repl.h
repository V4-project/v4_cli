#pragma once

#include <stddef.h>
#include <stdint.h>

#include "v4/vm_api.h"
#include "v4front/compile.h"

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @file repl.h
 * @brief Platform-independent REPL for V4 Forth VM
 *
 * Provides a simple read-eval-print loop implementation that can be
 * integrated into any platform (embedded, desktop, etc.).
 *
 * Features:
 * - Persistent word definitions across lines
 * - Stack preservation between evaluations
 * - Detailed error reporting
 * - Configurable memory limits
 */

/* ------------------------------------------------------------------------- */
/* Configuration                                                             */
/* ------------------------------------------------------------------------- */

/**
 * @brief REPL configuration structure
 *
 * Passed to v4_repl_create() to initialize the REPL context.
 */
typedef struct V4ReplConfig {
  struct Vm *vm;              /**< VM instance (must not be NULL) */
  V4FrontContext *front_ctx;  /**< Compiler context (must not be NULL) */
  size_t line_buffer_size;    /**< Maximum line length (0 = default: 512) */
} V4ReplConfig;

/**
 * @brief Opaque REPL context handle
 *
 * Created by v4_repl_create() and destroyed by v4_repl_destroy().
 */
typedef struct V4ReplContext V4ReplContext;

/* ------------------------------------------------------------------------- */
/* Lifecycle                                                                 */
/* ------------------------------------------------------------------------- */

/**
 * @brief Create a new REPL context
 *
 * @param config Configuration structure (must not be NULL)
 * @return REPL context pointer, or NULL on allocation failure
 *
 * @note The VM and compiler context must remain valid for the lifetime
 *       of the REPL context.
 */
V4ReplContext *v4_repl_create(const V4ReplConfig *config);

/**
 * @brief Destroy a REPL context
 *
 * Frees all resources associated with the REPL context.
 * Does not destroy the VM or compiler context (caller's responsibility).
 *
 * @param ctx REPL context (NULL-safe)
 */
void v4_repl_destroy(V4ReplContext *ctx);

/* ------------------------------------------------------------------------- */
/* Core REPL operations                                                      */
/* ------------------------------------------------------------------------- */

/**
 * @brief Process a single line of input
 *
 * Compiles and executes the given line, handling errors.
 * Stores word definitions for future lines.
 *
 * @param ctx  REPL context
 * @param line Input line (null-terminated string)
 * @return 0 on success, negative error code on failure
 *
 * Error codes:
 * - 0: Success
 * - Negative: Compilation or execution error (V4/V4-front error codes)
 *
 * @note This function does NOT print the stack or "ok" prompt.
 *       The caller should call v4_repl_print_stack() and print "ok"
 *       after successful evaluation.
 */
v4_err v4_repl_process_line(V4ReplContext *ctx, const char *line);

/**
 * @brief Reset REPL state
 *
 * Clears VM stacks and resets compiler context to initial state.
 * Does not clear VM memory or word dictionary.
 *
 * @param ctx REPL context
 */
void v4_repl_reset(V4ReplContext *ctx);

/**
 * @brief Reset only the word dictionary
 *
 * Clears all user-defined words but preserves stacks and memory.
 *
 * @param ctx REPL context
 */
void v4_repl_reset_dictionary(V4ReplContext *ctx);

/* ------------------------------------------------------------------------- */
/* Stack display helpers                                                     */
/* ------------------------------------------------------------------------- */

/**
 * @brief Get current data stack depth
 *
 * @param ctx REPL context
 * @return Number of elements on the data stack
 */
int v4_repl_stack_depth(const V4ReplContext *ctx);

/**
 * @brief Print stack contents to stdout
 *
 * Format: " ok [depth]: val1 val2 ... valN\n"
 * If stack is empty, prints " ok\n"
 *
 * @param ctx REPL context
 *
 * @note This function prints directly to stdout.
 *       For embedded systems, you may want to implement your own
 *       stack printing by using v4_repl_stack_depth() and
 *       vm_ds_peek_public() directly.
 */
void v4_repl_print_stack(const V4ReplContext *ctx);

/* ------------------------------------------------------------------------- */
/* Error handling helpers                                                    */
/* ------------------------------------------------------------------------- */

/**
 * @brief Get last error message
 *
 * Returns a pointer to the last error message string, or NULL if
 * no error has occurred since the last successful operation.
 *
 * @param ctx REPL context
 * @return Error message string (may be NULL)
 *
 * @note The returned pointer is valid until the next call to
 *       v4_repl_process_line() or v4_repl_destroy().
 */
const char *v4_repl_get_error(const V4ReplContext *ctx);

/* ------------------------------------------------------------------------- */
/* Version information                                                       */
/* ------------------------------------------------------------------------- */

/**
 * @brief Get REPL library version
 *
 * Format: 0xMMNNPP (MM = major, NN = minor, PP = patch)
 * Example: 0x000100 = version 0.1.0
 *
 * @return Version number as integer
 */
int v4_repl_version(void);

#ifdef __cplusplus
}
#endif
