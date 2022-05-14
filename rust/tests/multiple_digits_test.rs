use ean;

#[test]
fn test_seventy_two() {
  let numeral = ean::from(72);
  assert_eq!(2, numeral.digits.len());
  assert_eq!('\u{0667}', numeral.digits[0].to_char);
  assert_eq!('\u{0662}', numeral.digits[1].to_char);
}

#[test]
fn test_four_twenty() {
  let numeral = ean::from(420);
  assert_eq!(3, numeral.digits.len());
  assert_eq!('\u{0664}', numeral.digits[0].to_char);
  assert_eq!('\u{0662}', numeral.digits[1].to_char);
  assert_eq!('\u{0660}', numeral.digits[2].to_char);
}

#[test]
fn test_ltr() {
  let numeral = ean::from(42);
  assert_eq!(2, numeral.digits.len());
  assert_eq!("\u{0664}\u{0662}", format!("{}", numeral));
}

#[test]
fn test_to_western_numeral() {
  let numeral = ean::from(42);
  assert_eq!(numeral.to_western_numeral, 42);
}
