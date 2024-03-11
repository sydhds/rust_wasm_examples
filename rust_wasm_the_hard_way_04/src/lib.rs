use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[wasm_bindgen]
pub fn upper(msg: &str) -> String {
    msg.to_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::{console_log, wasm_bindgen_test};

    #[wasm_bindgen_test]
    fn test_add() {
        console_log!("Calling add function...");
        let result = add(2, 2);
        console_log!("Result: {:?}", result);
        assert_eq!(result, 4);
    }
    
    #[wasm_bindgen_test]
    fn test_upper() {
        console_log!("Calling upper function...");
        let result = upper("hey there!!");
        console_log!("Result: {}", result);
        assert_eq!(result, "HEY THERE!!");
    }
}
