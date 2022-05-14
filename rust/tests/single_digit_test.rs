use ean;

#[test]
fn test_zero() {
  let numeral = ean::from(0);
  assert_eq!(1, numeral.digits.len());
  assert_eq!('\u{0660}', numeral.digits[0].as_char);
}

#[test]
fn test_one() {
  let numeral = ean::from(1);
  assert_eq!(1, numeral.digits.len());
  assert_eq!('\u{0661}', numeral.digits[0].as_char);
}

#[test]
fn test_two() {
  let numeral = ean::from(2);
  assert_eq!(1, numeral.digits.len());
  assert_eq!('\u{0662}', numeral.digits[0].as_char);
}

#[test]
fn test_three() {
  let numeral = ean::from(3);
  assert_eq!(1, numeral.digits.len());
  assert_eq!('\u{0663}', numeral.digits[0].as_char);
}

#[test]
fn test_four() {
  let numeral = ean::from(4);
  assert_eq!(1, numeral.digits.len());
  assert_eq!('\u{0664}', numeral.digits[0].as_char);
}

#[test]
fn test_five() {
  let numeral = ean::from(5);
  assert_eq!(1, numeral.digits.len());
  assert_eq!('\u{0665}', numeral.digits[0].as_char);
}

#[test]
fn test_six() {
  let numeral = ean::from(6);
  assert_eq!(1, numeral.digits.len());
  assert_eq!('\u{0666}', numeral.digits[0].as_char);
}

#[test]
fn test_seven() {
  let numeral = ean::from(7);
  assert_eq!(1, numeral.digits.len());
  assert_eq!('\u{0667}', numeral.digits[0].as_char);
}

#[test]
fn test_eight() {
  let numeral = ean::from(8);
  assert_eq!(1, numeral.digits.len());
  assert_eq!('\u{0668}', numeral.digits[0].as_char);
}

#[test]
fn test_nine() {
  let numeral = ean::from(9);
  assert_eq!(1, numeral.digits.len());
  assert_eq!('\u{0669}', numeral.digits[0].as_char);
}
