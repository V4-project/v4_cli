# RECURSE Examples for V4-front v0.3.x

This document demonstrates the `RECURSE` keyword introduced in V4-front v0.3.0.

## What is RECURSE?

`RECURSE` allows a word to call itself recursively without needing to reference its own name. This is particularly useful for recursive algorithms.

## Example 1: Factorial

Calculate factorial: `n! = n × (n-1) × (n-2) × ... × 1`

```forth
: FACTORIAL
  DUP 1 >
  IF
    DUP 1 - RECURSE *
  THEN
;

( Test cases )
5 FACTORIAL  ( Expected: 120 )
10 FACTORIAL ( Expected: 3628800 )
0 FACTORIAL  ( Expected: 0 - edge case )
1 FACTORIAL  ( Expected: 1 )
```

## Example 2: Fibonacci

Calculate Fibonacci numbers: `F(n) = F(n-1) + F(n-2)`

```forth
: FIB
  DUP 2 <
  IF
    DROP 1
  ELSE
    DUP 1 - RECURSE
    SWAP 2 - RECURSE +
  THEN
;

( Test cases )
0 FIB  ( Expected: 1 )
1 FIB  ( Expected: 1 )
5 FIB  ( Expected: 8 )
7 FIB  ( Expected: 21 )
10 FIB ( Expected: 89 )
```

## Example 3: Greatest Common Divisor (GCD) - Iterative vs Recursive

### Iterative approach (traditional)
```forth
: GCD-ITER
  BEGIN
    DUP 0 >
  WHILE
    SWAP OVER MOD
  REPEAT
  DROP
;
```

### Recursive approach with RECURSE
```forth
: GCD
  DUP 0 =
  IF
    DROP
  ELSE
    SWAP OVER MOD RECURSE
  THEN
;

( Test cases )
48 18 GCD    ( Expected: 6 )
100 35 GCD   ( Expected: 5 )
```

## Example 4: Power Function

Calculate `base^exponent` recursively:

```forth
: POWER
  DUP 0 =
  IF
    DROP DROP 1
  ELSE
    1 -
    SWAP DUP ROT
    RECURSE *
  THEN
;

( Test cases )
2 3 POWER  ( Expected: 8 = 2^3 )
5 2 POWER  ( Expected: 25 = 5^2 )
10 0 POWER ( Expected: 1 = 10^0 )
```

## Example 5: Sum of Digits

Sum all digits of a positive integer:

```forth
: SUM-DIGITS
  DUP 10 <
  IF
    ( Single digit, return as-is )
  ELSE
    ( Recursively sum: current digit + sum of remaining digits )
    DUP 10 MOD
    SWAP 10 / RECURSE +
  THEN
;

( Test cases )
123 SUM-DIGITS   ( Expected: 6 = 1+2+3 )
9876 SUM-DIGITS  ( Expected: 30 = 9+8+7+6 )
```

## Example 6: Countdown

Simple countdown demonstration:

```forth
: COUNTDOWN
  DUP 0 >
  IF
    DUP .
    1 - RECURSE
  THEN
;

( Test )
5 COUNTDOWN  ( Expected: prints 5 4 3 2 1 )
```

## Example 7: Ackermann Function

A computationally intensive recursive function:

```forth
: ACKERMANN
  OVER 0 =
  IF
    NIP 1 +
  ELSE
    DUP 0 =
    IF
      DROP 1 - 1 RECURSE
    ELSE
      OVER 1 - ROT 1 - SWAP RECURSE SWAP RECURSE
    THEN
  THEN
;

( Warning: Grows very quickly! )
( Test with small values only )
1 2 ACKERMANN  ( Expected: 4 )
2 2 ACKERMANN  ( Expected: 7 )
```

## Tips for Using RECURSE

1. **Base case first**: Always check the termination condition before recursing
2. **Stack management**: Be careful with stack manipulation in recursive calls
3. **Performance**: Recursive solutions can be slower than iterative ones
4. **Testing**: Start with small input values and verify correctness
5. **Stack depth**: V4 VM has finite stack space; very deep recursion may fail

## Comparison: RECURSE vs Named Recursion

### Without RECURSE (not allowed in V4)
```forth
( This does NOT work - can't reference name during definition )
: FACTORIAL-BAD
  DUP 1 > IF DUP 1 - FACTORIAL-BAD * THEN
;
```

### With RECURSE (correct)
```forth
: FACTORIAL
  DUP 1 > IF DUP 1 - RECURSE * THEN
;
```

## Performance Considerations

| Example | Complexity | Notes |
|---------|-----------|-------|
| Factorial | O(n) | Simple tail recursion |
| Fibonacci | O(2^n) | Exponential - slow for large n |
| GCD | O(log min(a,b)) | Efficient |
| Power | O(n) | Can be optimized |
| Sum Digits | O(log n) | Fast |
| Ackermann | O(huge) | Grows extremely fast |

For production code, consider iterative alternatives for performance-critical paths.

## Testing Your RECURSE Words

To test a RECURSE word interactively in V4-repl:

```bash
$ ./build/v4-repl
v4> : FACTORIAL DUP 1 > IF DUP 1 - RECURSE * THEN ;
 ok

v4> 5 FACTORIAL
 ok [1]: 120

v4> .stack
Data Stack (depth: 1):
  [0]: 120 (0x00000078)
 ok [1]: 120
```
