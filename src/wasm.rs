extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use crate::bidirectional_scheme;

#[wasm_bindgen]
pub fn min_bs(s: &str) -> String {
    match bidirectional_scheme::calc_min_bs(s) {
        Some(res) => res,
        None => "error".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::bidirectional_scheme;
    use crate::wasm::min_bs;

    #[test]
    fn bs_test() {
        let s = "ababbaababaab";
        println!("wasm: {}", min_bs(s));
    }
}
