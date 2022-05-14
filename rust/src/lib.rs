use std::fmt;

static DIGITS: [char; 10] = [
    '\u{0660}', '\u{0661}', '\u{0662}',
    '\u{0663}', '\u{0664}', '\u{0665}',
    '\u{0666}', '\u{0667}', '\u{0668}',
    '\u{0669}',
];

#[derive(Debug)]
pub struct Numeral {
    pub digits: Vec<Digit>
}

impl fmt::Display for Numeral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let iter = self.digits.iter();
        let str: String = .map(|digit| { digit.to_char })
                          .collect();
        write!(f, "{}", str)
    }
}

impl PartialEq<usize> for Numeral {
    fn eq(&self, other: &usize) -> bool {
        let iter = self.digits.iter();
        let number: usize = iter.map(|d| { d.to_western_digit.to_string() })
                                .collect::<String>().parse().unwrap();
        number == *other
    }
}

#[derive(Debug)]
pub struct Digit {
    pub to_char: char,
    pub to_western_digit: usize,
}

pub fn from(num: usize) -> Numeral {
    let str = num.to_string();
    let mut numeral = Numeral { digits: Vec::with_capacity(str.len()) };

    for char in str.chars() {
        let w_digit = char.to_digit(10).unwrap() as usize;
        numeral.digits.push(Digit {
            to_char: DIGITS[w_digit].clone(),
            to_western_digit: w_digit,
        });
    }

    numeral
}

