use ean;
use wasm_bindgen::prelude::*;
use js_sys::Array;

#[wasm_bindgen]
pub struct Numeral {
    pub to_western_numeral: usize,
    _digits: Vec<Digit>
}

#[wasm_bindgen]
impl Numeral {
    #[wasm_bindgen]
    pub fn digits(&self) -> Array {
        let iter = self._digits.clone().into_iter();
        iter.map(JsValue::from).collect()
    }
}

#[derive(Clone)]
#[wasm_bindgen]
pub struct Digit {
    pub to_char: char,
    pub to_western_digit: usize
}

#[wasm_bindgen]
pub fn from(num: usize) -> Numeral {
    let numeral = ean::from(num);
    let digits  = make_digits(numeral.digits);
    Numeral {
        to_western_numeral: num,
        _digits: digits
    }
}

fn make_digits(digits: Vec<ean::Digit>) -> Vec<Digit> {
    digits.iter().map(|digit| {
        Digit {
            to_char: digit.to_char,
            to_western_digit: digit.to_western_digit
        }
    }).collect()
}
