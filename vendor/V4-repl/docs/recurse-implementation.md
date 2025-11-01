# RECURSE Implementation Guide

## Overview

`RECURSE` is a Forth word that allows a word to call itself during its own definition.

## Problem

In Forth, you cannot reference a word by name while defining it:

```forth
❌ : FACTORIAL
  DUP 1 >
  IF DUP 1 - FACTORIAL *  ( Error: FACTORIAL not yet defined! )
  ELSE DROP 1
  THEN
;
```

## Solution: RECURSE

```forth
✅ : FACTORIAL
  DUP 1 >
  IF DUP 1 - RECURSE *    ( RECURSE refers to current word )
  ELSE DROP 1
  THEN
;
```

## Implementation in V4-front

### Required Changes

#### 1. Add Error Code

In `include/v4front/errors.h`:

```c
typedef enum {
  // ... existing errors ...
  V4FRONT_ERR_RECURSE_OUTSIDE_DEFINITION = -20,
} v4front_err_t;
```

#### 2. Track Current Word ID During Compilation

In `src/compile.cpp`, modify the compilation state:

```cpp
// Add to compilation function parameters
static FrontErr compile_internal(
  // ... existing params ...
  int* current_word_id,  // NEW: Track current word being defined
  // ...
)
```

#### 3. Implement RECURSE Token Handling

In the token processing loop:

```cpp
// After checking for other special tokens (IF, THEN, etc.)

// Check for RECURSE
if (str_eq_ci(token, "RECURSE")) {
  // Must be inside a word definition
  if (!in_definition) {
    if (error_pos) *error_pos = token_start;
    cleanup_compile_state(bc, word_bc, word_dict, word_count);
    return FrontErr::RecurseOutsideDefinition;
  }

  // Emit CALL instruction to current word
  FrontErr err;
  if ((err = append_byte(current_bc, current_bc_size, current_bc_cap,
                         static_cast<uint8_t>(v4::Op::CALL))) != FrontErr::OK) {
    cleanup_compile_state(bc, word_bc, word_dict, word_count);
    return err;
  }

  // Emit word ID (current_word_id)
  uint32_t wid = *current_word_id;
  for (int i = 0; i < 4; i++) {
    if ((err = append_byte(current_bc, current_bc_size, current_bc_cap,
                           (wid >> (i * 8)) & 0xFF)) != FrontErr::OK) {
      cleanup_compile_state(bc, word_bc, word_dict, word_count);
      return err;
    }
  }

  continue;
}
```

#### 4. Set current_word_id When Starting Definition

In `handle_colon_start`:

```cpp
static FrontErr handle_colon_start(
  // ... existing params ...
  int* current_word_id,  // NEW
  int word_count         // NEW
) {
  // ... existing code ...

  // Set current word ID to the index this word will have
  *current_word_id = word_count;

  // ... rest of existing code ...
}
```

### Alternative: Simpler Self-Reference Approach

Instead of tracking the word ID, V4-front can:

1. Add the word to the dictionary **immediately** when `:` is encountered
2. Mark it as "incomplete"
3. Allow RECURSE to look it up normally
4. Finalize it when `;` is encountered

This requires less state tracking but needs careful handling of incomplete words.

## Testing

### Test Case 1: Basic Recursion

```forth
v4> : FACTORIAL
...   DUP 1 >
...   IF DUP 1 - RECURSE *
...   ELSE DROP 1
...   THEN
... ;
 ok

v4> 5 FACTORIAL .
120 ok
```

### Test Case 2: Mutual Recursion (Not Supported)

RECURSE only supports direct recursion, not mutual recursion:

```forth
( This won't work )
: EVEN? DUP 0 = IF DROP 1 ELSE 1 - ODD? THEN ;
: ODD?  DUP 0 = IF DROP 0 ELSE 1 - EVEN? THEN ;

( Would need forward declarations )
```

### Test Case 3: RECURSE Outside Definition (Error)

```forth
v4> RECURSE
Error: RECURSE used outside word definition
```

### Test Case 4: Countdown

```forth
v4> : COUNTDOWN
...   DUP .
...   DUP 0 >
...   IF 1 - RECURSE
...   ELSE DROP
...   THEN
... ;
 ok

v4> 5 COUNTDOWN
5 4 3 2 1 0 ok
```

## VM Considerations

The V4 VM must support:
- `CALL` instruction with word ID
- Proper return stack management for recursive calls
- Stack depth limits to prevent stack overflow

## Current Status

**Not yet implemented** in V4-front or V4-repl.

## Priority

**Medium** - Useful feature but workarounds exist (define helper words).

## Workaround (Current)

Until RECURSE is implemented, use named recursion with forward declarations or restructure code:

```forth
( Instead of FACTORIAL with RECURSE )
( Use tail recursion with helper word )

: FACT-HELPER ( n acc -- result )
  OVER 1 >
  IF OVER * SWAP 1 - SWAP FACT-HELPER
  ELSE NIP
  THEN
;

: FACTORIAL ( n -- n! )
  1 FACT-HELPER
;
```

## References

- ANS Forth Standard: 6.1.2120 RECURSE
- V4 Opcodes: `v4::Op::CALL`
- V4-front compiler: `src/compile.cpp`
