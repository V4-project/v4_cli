#include "meta_commands.hpp"

#include <cstdio>
#include <cstring>

MetaCommands::MetaCommands(struct Vm* vm, V4FrontContext* ctx) : vm_(vm), ctx_(ctx) {}

bool MetaCommands::execute(const char* line) {
  // Skip leading whitespace
  while (*line == ' ' || *line == '\t') {
    line++;
  }

  // Check if it's a meta-command
  if (*line != '.') {
    return false;  // Not a meta-command
  }

  line++;  // Skip the '.'

  // Match command
  if (strncmp(line, "words", 5) == 0 && (line[5] == '\0' || line[5] == ' ')) {
    cmd_words();
  } else if (strncmp(line, "stack", 5) == 0 && (line[5] == '\0' || line[5] == ' ')) {
    cmd_stack();
  } else if (strncmp(line, "reset", 5) == 0 && (line[5] == '\0' || line[5] == ' ')) {
    cmd_reset();
  } else if (strncmp(line, "memory", 6) == 0 && (line[6] == '\0' || line[6] == ' ')) {
    cmd_memory();
  } else if (strncmp(line, "help", 4) == 0 && (line[4] == '\0' || line[4] == ' ')) {
    cmd_help();
  } else if (strncmp(line, "version", 7) == 0 && (line[7] == '\0' || line[7] == ' ')) {
    cmd_version();
  } else {
    // Unknown meta-command
    printf("Unknown meta-command: .%s\n", line);
    printf("Type .help for available commands\n");
  }

  return true;
}

void MetaCommands::cmd_words() {
  int count = v4front_context_get_word_count(ctx_);

  if (count == 0) {
    printf("No words defined.\n");
    return;
  }

  printf("Defined words (%d):\n", count);
  for (int i = 0; i < count; i++) {
    const char* name = v4front_context_get_word_name(ctx_, i);
    if (name) {
      printf("  %s\n", name);
    }
  }
}

void MetaCommands::cmd_stack() {
  int depth = vm_ds_depth_public(vm_);

  printf("Data Stack (depth: %d):\n", depth);
  if (depth == 0) {
    printf("  <empty>\n");
  } else {
    // Print from bottom to top (index 0 = bottom)
    for (int i = depth - 1; i >= 0; i--) {
      v4_i32 val = vm_ds_peek_public(vm_, i);
      printf("  [%d]: %d (0x%08X)\n", depth - 1 - i, val, (unsigned int) val);
    }
  }

  // TODO: Add return stack display when V4-core API is available
  printf("\nReturn Stack: <not yet implemented>\n");
}

void MetaCommands::cmd_reset() {
  vm_reset(vm_);
  v4front_context_reset(ctx_);
  printf("VM and compiler context reset.\n");
}

void MetaCommands::cmd_memory() {
  // TODO: Implement when V4-core provides memory usage API
  printf("Memory usage information:\n");
  printf("  VM memory size: (not yet available from V4-core)\n");
  printf("  Data stack depth: %d\n", vm_ds_depth_public(vm_));
  printf("  Return stack depth: (API not yet available)\n");
  printf("  Registered words: %d\n", v4front_context_get_word_count(ctx_));
}

void MetaCommands::cmd_help() {
  printf("V4 REPL Help\n");
  printf("════════════════════════════════════════════════════════════════\n\n");

  printf("Meta-commands:\n");
  printf("  .words     - List all defined words\n");
  printf("  .stack     - Show detailed stack contents (hex and decimal)\n");
  printf("  .reset     - Reset VM and compiler context\n");
  printf("  .memory    - Show memory usage statistics\n");
  printf("  .help      - Show this help message\n");
  printf("  .version   - Show REPL and component versions\n");

  printf("\nPASTE mode (multi-line input):\n");
  printf("  <<<        - Enter PASTE mode for multi-line definitions\n");
  printf("  >>>        - Exit PASTE mode and compile buffered input\n");

  printf("\nControl:\n");
  printf("  Ctrl+D     - Exit REPL\n");
  printf("  Ctrl+C     - Interrupt execution (if supported)\n");
  printf("  bye / quit - Exit REPL\n");

  printf("\nBasic Forth:\n");
  printf("  Numbers    - Push values to stack (e.g., '42')\n");
  printf("  + - * /    - Arithmetic operations\n");
  printf("  DUP        - Duplicate top of stack\n");
  printf("  DROP       - Remove top of stack\n");
  printf("  SWAP       - Swap top two values\n");
  printf("  .          - Print and pop top of stack\n");

  printf("\nWord definitions:\n");
  printf("  : NAME ... ; - Define a new word\n");
  printf("  Example: : SQUARE DUP * ;\n");

  printf("\n════════════════════════════════════════════════════════════════\n");
}

void MetaCommands::cmd_version() {
  printf("V4 REPL v0.2.0\n");
  printf("════════════════════════════════════════════════════════════════\n");
  printf("Components:\n");
  printf("  V4-core:   (version API not yet available)\n");
  printf("  V4-front:  (version API not yet available)\n");
  printf("  linenoise: integrated\n");
  printf("\nBuild configuration:\n");
#ifdef WITH_FILESYSTEM
  printf("  Filesystem: enabled (history support)\n");
#else
  printf("  Filesystem: disabled (no history support)\n");
#endif
  printf("  C++ standard: C++17\n");
  printf("  Exceptions: disabled (-fno-exceptions)\n");
  printf("  RTTI: disabled (-fno-rtti)\n");
  printf("════════════════════════════════════════════════════════════════\n");
}
