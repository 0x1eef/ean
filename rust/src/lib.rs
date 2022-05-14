use std::fmt;

static DIGITS: [char; 10] = [
    '\u{0660}', '\u{0661}', '\u{0662}',
    '\u{0663}', '\u{0664}', '\u{0665}',
    '\u{0666}', '\u{0667}', '\u{0668}',
    '\u{0669}',
];

pub struct Digit {
    pub as_char: char,
    pub as_wdigit: usize,
}

pub struct Numeral {
    pub digits: Vec<Digit>
}

impl fmt::Display for Numeral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str: String = self
                          .digits
                          .iter()
                          .map(|digit| { digit.as_char })
                          .collect();
        write!(f, "{}", str)
    }
}

pub fn from(num: usize) -> Numeral {
    let str = num.to_string();
    let mut numeral = Numeral {
        digits: Vec::with_capacity(str.len())
    };

    for char in str.chars() {
        let w_digit = char.to_digit(10).unwrap() as usize;
        numeral.digits.push(Digit {
            as_char: DIGITS[w_digit].clone(),
            as_wdigit: w_digit,
        });
    }

    numeral
}

