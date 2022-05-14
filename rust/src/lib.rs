use std::fmt;

static DIGITS: [char; 10] = [
    '\u{0660}', '\u{0661}', '\u{0662}',
    '\u{0663}', '\u{0664}', '\u{0665}',
    '\u{0666}', '\u{0667}', '\u{0668}',
    '\u{0669}',
];

#[derive(Clone)]
#[derive(Debug)]
pub struct Numeral {
    pub digits: Vec<Digit>,
    pub to_western_numeral: usize
}

impl fmt::Display for Numeral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let iter = self.digits.iter();
        let str: String = iter.map(|digit| { digit.to_char })
                              .collect();
        write!(f, "{}", str)
    }
}

impl PartialEq<usize> for Numeral {
    fn eq(&self, other: &usize) -> bool {
        self.to_western_numeral == *other
    }
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Digit {
    pub to_char: char,
    pub to_western_digit: usize,
}

pub fn from(num: usize) -> Numeral {
    let str = num.to_string();
    let mut numeral = Numeral {
        digits: Vec::with_capacity(str.len()),
        to_western_numeral: num
    };

    for char in str.chars() {
        let w_digit = char.to_digit(10).unwrap() as usize;
        numeral.digits.push(Digit {
            to_char: DIGITS[w_digit].clone(),
            to_western_digit: w_digit
        });
    }

    numeral
}

