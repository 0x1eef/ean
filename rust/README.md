# ean.rs

The ean.rs Rust library can convert Western Arabic Numerals to
Eastern Arabic Numerals. Western Arabic Numerals are represented
by the digits between `0` and `9`, while Eastern Arabic Numerals are
represented by the digits between `٠` (zero) and `٩` (nine).

## Examples

**1. Obtain an Eastern Arabic Numeral**

The following examples takes the Western Arabic Numeral, `120`,
and converts it to its Eastern Arabic Numeral counterpart. Note that,
both western and eastern arabic numerals should be read from left-to-right
(LTR), despite Arabic being read from right-to-left (RTL). Some terminals
don't follow that rule though, and might print the eastern numerals RTL,
as they would with other Arabic characters.

```rust
use ean;

fn main() {
    let numeral = ean::from(120);
    println!("{}", numeral);
}
```

**2. Obtain the digits of an Eastern Arabic Numeral**

In the following example, we see how the digits that make
up an Eastern Arabic Numeral can be accessed individually
through the "digits" field available on instances of the
`ean::Numeral` struct.

```rust
use ean;

fn main() {
    let numeral = ean::from(42);
    let digits  = numeral.digits.iter();
    for (i, digit) in digits.enumerate() {
      println!("Digit {} is {}", i, digit.to_char);
    }
}
```

**3. Compare eastern numerals with western numerals**

In the following example, we see how an Eastern Arabic Numeral
can be compared with a Western Arabic Numeral using the equality
operator.

```rust
use ean;

fn main() {
  let numeral = ean::from(42);
  /* This expression evaluates to true */
  if numeral == 42 {
    println!("{} is equal to {}", numeral, 42);
  }
}
```

## License

This software is released under the MIT license, see
[./LICENSE.txt](./LICENSE.txt) for details.
