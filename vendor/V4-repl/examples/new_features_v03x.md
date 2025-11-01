# V4-front v0.3.x New Features Examples

Complete examples for all new features introduced in V4-front v0.3.0 and v0.3.1.

## Table of Contents

- [Arithmetic Operations](#arithmetic-operations)
- [Bitwise Operations](#bitwise-operations)
- [Stack Manipulation](#stack-manipulation)
- [Comparison Operations](#comparison-operations)
- [Utilities](#utilities)
- [Recursion](#recursion)

---

## Arithmetic Operations

### Increment and Decrement (v0.3.0)

```forth
( 1+ : Increment top of stack )
5 1+     ( Result: 6 )
-1 1+    ( Result: 0 )
99 1+    ( Result: 100 )

( 1- : Decrement top of stack )
5 1-     ( Result: 4 )
0 1-     ( Result: -1 )
100 1-   ( Result: 99 )

( Practical use: loop counters )
: COUNT-DOWN
  10
  BEGIN
    DUP .
    1 -
    DUP 0 <
  UNTIL
  DROP
;
```

### Unsigned Division and Modulo (v0.3.0)

```forth
( U/ : Unsigned division )
10 3 U/       ( Result: 3 )
17 5 U/       ( Result: 3 )

( UMOD : Unsigned modulo )
10 3 UMOD     ( Result: 1 )
17 5 UMOD     ( Result: 2 )

( Useful for dealing with large positive numbers )
4294967295 2 U/     ( Result: 2147483647 - unsigned interpretation )
```

---

## Bitwise Operations (v0.3.0)

### Left Shift (LSHIFT)

```forth
( Multiply by powers of 2 )
1 1 LSHIFT    ( 1 << 1 = 2 )
1 3 LSHIFT    ( 1 << 3 = 8 )
5 2 LSHIFT    ( 5 << 2 = 20 )

( Build bit masks )
: BIT ( n -- mask )
  1 SWAP LSHIFT
;

3 BIT    ( Result: 8 = 00001000 binary )
```

### Logical Right Shift (RSHIFT)

```forth
( Divide by powers of 2 - unsigned )
8 1 RSHIFT    ( 8 >> 1 = 4 )
32 3 RSHIFT   ( 32 >> 3 = 4 )

( Extract high bits )
: HIGH-NIBBLE ( byte -- nibble )
  4 RSHIFT
  15 AND    ( Mask to 4 bits )
;

237 HIGH-NIBBLE  ( Result: 14 = 0xE )
```

### Arithmetic Right Shift (ARSHIFT)

```forth
( Divide by powers of 2 - signed, preserves sign )
8 1 ARSHIFT     ( 8 >> 1 = 4 )
-8 1 ARSHIFT    ( -8 >> 1 = -4, sign preserved )
-16 2 ARSHIFT   ( -16 >> 2 = -4 )

( Compare with RSHIFT )
-8 1 RSHIFT     ( Result: large positive number - sign not preserved )
-8 1 ARSHIFT    ( Result: -4 - sign preserved )
```

---

## Stack Manipulation

### Single-Cell Operations (v0.3.0)

#### ROT (Rotate)

```forth
( ROT : a b c -- b c a )
1 2 3 ROT    ( Result: 2 3 1 )

( Useful for reordering before operations )
: DIVIDE-THREE ( a b c -- c/b/a )
  ROT ROT /
  SWAP /
;
```

#### NIP (Remove Second)

```forth
( NIP : a b -- b )
10 20 NIP    ( Result: 20 )

( Keep result, discard intermediate )
5 2 * NIP    ( Result: 10, removes original 5 )
```

#### TUCK (Insert Copy)

```forth
( TUCK : a b -- b a b )
5 10 TUCK    ( Result: 10 5 10 )

( Compare before operation )
: SAFE-DIV
  TUCK 0 =
  IF
    DROP DROP 0  ( Avoid division by zero )
  ELSE
    /
  THEN
;
```

### Double-Cell Operations (v0.3.1)

#### 2DUP (Duplicate Two)

```forth
( 2DUP : a b -- a b a b )
1 2 2DUP     ( Result: 1 2 1 2 )

( Useful for operations needing both values twice )
: RECT-AREA-AND-PERIMETER ( w h -- area perimeter )
  2DUP *          ( Calculate area )
  ROT ROT +  2 *  ( Calculate perimeter )
;

5 3 RECT-AREA-AND-PERIMETER  ( Result: 15 16 )
```

#### 2DROP (Drop Two)

```forth
( 2DROP : a b -- )
1 2 3 4 2DROP    ( Result: 1 2 )

( Clean up unwanted pairs )
: IGNORE-RESULT ( a b -- )
  2DROP
;
```

#### 2SWAP (Swap Two Pairs)

```forth
( 2SWAP : a b c d -- c d a b )
1 2 3 4 2SWAP    ( Result: 3 4 1 2 )

( Useful for complex fractions )
: FRACTION-ADD ( n1 d1 n2 d2 -- n3 d3 )
  ( numerator1, denominator1, numerator2, denominator2 )
  ( Add fractions: a/b + c/d = (a*d + c*b) / (b*d) )
  2SWAP OVER * ROT ROT * +
  SWAP ROT *
;
```

#### 2OVER (Copy Second Pair)

```forth
( 2OVER : a b c d -- a b c d a b )
1 2 3 4 2OVER    ( Result: 1 2 3 4 1 2 )
```

---

## Comparison Operations

### Zero Comparison (v0.3.1)

```forth
( 0= : Test if zero )
0 0=     ( Result: TRUE (-1) )
5 0=     ( Result: FALSE (0) )

( 0< : Test if negative )
-5 0<    ( Result: TRUE (-1) )
5 0<     ( Result: FALSE (0) )

( 0> : Test if positive )
5 0>     ( Result: TRUE (-1) )
-5 0>    ( Result: FALSE (0) )
0 0>     ( Result: FALSE (0) )

( Practical use: sign checking )
: ABS-MANUAL ( n -- |n| )
  DUP 0<
  IF
    NEGATE
  THEN
;
```

### Unsigned Comparison (v0.3.0)

```forth
( U< : Unsigned less than )
5 10 U<        ( Result: TRUE (-1) )
10 5 U<        ( Result: FALSE (0) )

( U<= : Unsigned less than or equal )
5 5 U<=        ( Result: TRUE (-1) )
5 10 U<=       ( Result: TRUE (-1) )
10 5 U<=       ( Result: FALSE (0) )

( Important for unsigned ranges )
-1 1 U<        ( Result: FALSE - -1 is large unsigned )
-1 1 <         ( Result: TRUE - -1 is negative signed )
```

### Boolean Constants (v0.3.1)

```forth
( TRUE : -1 (all bits set) )
TRUE     ( Result: -1 )

( FALSE : 0 (all bits clear) )
FALSE    ( Result: 0 )

( Use in conditions )
: IS-EVEN? ( n -- flag )
  2 MOD 0 =
;

5 IS-EVEN?  ( Result: FALSE )
4 IS-EVEN?  ( Result: TRUE )
```

---

## Utilities

### NEGATE (v0.3.0)

```forth
( NEGATE : Sign negation )
5 NEGATE     ( Result: -5 )
-7 NEGATE    ( Result: 7 )
0 NEGATE     ( Result: 0 )

( Implementation equivalent: 0 SWAP - )
```

### ABS (v0.3.0)

```forth
( ABS : Absolute value )
5 ABS        ( Result: 5 )
-5 ABS       ( Result: 5 )
0 ABS        ( Result: 0 )

( Distance calculation )
: DISTANCE ( a b -- |a-b| )
  - ABS
;

10 3 DISTANCE   ( Result: 7 )
3 10 DISTANCE   ( Result: 7 )
```

### MIN and MAX (v0.3.0)

```forth
( MIN : Minimum of two values )
3 7 MIN      ( Result: 3 )
-5 -2 MIN    ( Result: -5 )

( MAX : Maximum of two values )
3 7 MAX      ( Result: 7 )
-5 -2 MAX    ( Result: -2 )

( Clamping values to range )
: CLAMP ( n min max -- n' )
  ROT    ( min max n )
  MAX    ( min n' where n' >= n )
  MIN    ( n'' where n'' <= max )
;

50 0 100 CLAMP   ( Result: 50 )
-10 0 100 CLAMP  ( Result: 0 )
200 0 100 CLAMP  ( Result: 100 )
```

### ?DUP (Conditional Duplicate) (v0.3.0)

```forth
( ?DUP : Duplicate if non-zero )
5 ?DUP       ( Result: 5 5 )
0 ?DUP       ( Result: 0 )

( Useful for conditional operations )
: SAFE-RECIPROCAL ( n -- 1/n or 0 )
  ?DUP
  IF
    1 SWAP /
  ELSE
    0
  THEN
;

4 SAFE-RECIPROCAL    ( Result: 0 - integer division )
0 SAFE-RECIPROCAL    ( Result: 0 - avoid divide by zero )
```

---

## Recursion (v0.3.0)

See [recurse_examples.md](recurse_examples.md) for detailed RECURSE examples including:
- Factorial
- Fibonacci
- GCD
- Power
- And more!

---

## Memory Access (v0.3.0)

### Byte Operations (C@ / C!)

```forth
( C! : Store byte )
( C@ : Fetch byte )

variable BYTE-VAR
255 BYTE-VAR C!    ( Store byte )
BYTE-VAR C@        ( Result: 255 )

( Useful for character handling )
```

### Halfword Operations (W@ / W!)

```forth
( W! : Store 16-bit value )
( W@ : Fetch 16-bit value )

variable HALF-VAR
65535 HALF-VAR W!  ( Store halfword )
HALF-VAR W@        ( Result: 65535 )

( Useful for compact data structures )
```

---

## Complete Example: Using Multiple New Features

### Example: Calculate Statistics

```forth
: STATS ( a b c -- min max avg )
  ( Calculate min, max, and average of three numbers )

  ( Calculate sum for average )
  ROT 2DUP +       ( b c a a+b )
  ROT +            ( a+b c sum )
  SWAP ROT         ( sum a b c )

  ( Find min and max )
  2DUP MIN         ( sum a b c min(b,c) )
  ROT MIN          ( sum b c min3 )
  SWAP ROT ROT     ( sum min3 b c )
  MAX ROT MAX      ( sum min3 max3 )

  ( Calculate average )
  ROT 3 /          ( min3 max3 avg )
  ROT ROT          ( avg min3 max3 )
  SWAP             ( avg max3 min3 )
  SWAP             ( avg min3 max3 )
;

10 20 5 STATS  ( Expected: 5 20 11 )
```

---

## Testing New Features

To test these features in V4-repl:

```bash
$ ./build/v4-repl
v4> ( Try any example from above )
v4> 5 1+
 ok [1]: 6

v4> 1 3 LSHIFT
 ok [1]: 8

v4> 3 7 MIN
 ok [1]: 3

v4> TRUE
 ok [1]: -1
```

All features are available immediately in the REPL with no additional setup required!
