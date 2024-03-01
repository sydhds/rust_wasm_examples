// no_mangle is here to make sure the compiler does not remove our function
#[no_mangle]
// extern "C" -> use C ABI, rust ABI is not stable
pub extern "C" fn add_1(left: usize, right: usize) -> usize {
    left + right
}

// End add_1

#[link(wasm_import_module = "Math")]
// Link to external lib - ok for WASM
extern "C" {
    fn random() -> f64;
}

#[no_mangle]
pub extern "C" fn add_2(left: f64, right: f64) -> f64 {
    left + right + unsafe { random() }
}

// End add_2

mod math {
    mod math_js {
        #[link(wasm_import_module = "Math")]
        extern "C" {
            pub fn random() -> f64;
        }
    }

    pub fn random() -> f64 {
        unsafe { math_js::random() }
    }
}

// Modify export name
#[export_name = "add_3"]
pub extern "C" fn add_with_math_random(left: f64, right: f64) -> f64 {
    left + right + math::random()
}

// End add_3

//

const _: () = {
    #[link_section = "MySection1"]
    static SECTION_CONTENT: [u8; 11] = *b"hello world";
};

// End custom section

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_1(2, 2);
        assert_eq!(result, 4);
    }
}
