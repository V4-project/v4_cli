#pragma once

#include <v4/vm_api.h>
#include <v4front/compile.h>

#include "meta_commands.hpp"

/**
 * @brief Interactive REPL for V4 Forth VM
 *
 * Provides a read-eval-print loop with optional history support
 * when compiled with WITH_FILESYSTEM=1.
 *
 * Features:
 * - Persistent word definitions across lines
 * - Stack preservation
 * - Detailed error messages with position information
 * - Meta-commands for REPL control (.words, .stack, .reset, etc.)
 */
class Repl {
 public:
  /**
   * @brief Construct a new REPL instance
   *
   * Initializes VM and compiler context with default configuration.
   * Loads history if filesystem support is enabled.
   */
  Repl();

  /**
   * @brief Destroy the REPL instance
   *
   * Saves history (if enabled) and frees all resources.
   */
  ~Repl();

  /**
   * @brief Run the REPL loop
   *
   * Continues until user enters Ctrl+D or 'bye' command.
   *
   * @return Exit code (0 = success)
   */
  int run();

 private:
  struct Vm* vm_;
  V4FrontContext* compiler_ctx_;
  uint8_t vm_memory_[16384];  // 16KB RAM for VM
  MetaCommands meta_cmds_;

  // Track word definition buffers (must not be freed while VM is alive)
  V4FrontBuf* word_bufs_;
  int word_buf_count_;
  int word_buf_capacity_;

  // PASTE mode state
  bool paste_mode_;
  char* paste_buffer_;
  int paste_buffer_size_;
  int paste_buffer_capacity_;

#ifdef WITH_FILESYSTEM
  char history_path_[256];
  void init_history();
  void save_history();
#endif

  /**
   * @brief Print current data stack contents
   *
   * Format: " ok [depth]: val1 val2 ... valN"
   */
  void print_stack();

  /**
   * @brief Print error message with context
   *
   * @param msg Error message string
   * @param code Optional error code (default: 0)
   */
  void print_error(const char* msg, int code = 0);

  /**
   * @brief Evaluate a single line of input
   *
   * Compiles and executes the line, handling errors.
   *
   * @param line Input string to evaluate
   * @return 0 on success, -1 on error, 1 to exit
   */
  int eval_line(const char* line);

  /**
   * @brief Check if line is a PASTE mode marker (<<< or >>>)
   */
  bool is_paste_marker(const char* line);

  /**
   * @brief Enter PASTE mode for multi-line input
   */
  void enter_paste_mode();

  /**
   * @brief Exit PASTE mode and compile buffered input
   */
  void exit_paste_mode();

  /**
   * @brief Get the current prompt string
   */
  const char* get_prompt() const;
};
