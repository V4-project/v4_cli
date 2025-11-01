#pragma once

#include <v4/vm_api.h>
#include <v4front/compile.h>

/**
 * @brief Meta-command handler for V4 REPL
 *
 * Provides dot-commands for inspecting and controlling the REPL state:
 * - .words    : List all defined words
 * - .stack    : Show detailed stack contents
 * - .reset    : Reset VM and compiler context
 * - .memory   : Show memory usage (TODO: requires V4-core API)
 * - .help     : Show help message
 * - .version  : Show version information
 */
class MetaCommands {
 public:
  /**
   * @brief Construct a new MetaCommands handler
   *
   * @param vm Pointer to VM instance
   * @param ctx Pointer to compiler context
   */
  MetaCommands(struct Vm* vm, V4FrontContext* ctx);

  /**
   * @brief Execute a meta-command if the line starts with '.'
   *
   * @param line Input line to check and execute
   * @return true if line was a meta-command (executed), false otherwise
   */
  bool execute(const char* line);

 private:
  struct Vm* vm_;
  V4FrontContext* ctx_;

  void cmd_words();
  void cmd_stack();
  void cmd_reset();
  void cmd_memory();
  void cmd_help();
  void cmd_version();
};
