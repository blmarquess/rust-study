mod reverse_string;
use crate::reverse_string::{reverse_string_for_char, reverse_string_native_method};


fn main() {
    let string_test="malta";
    let test1 = reverse_string_native_method(&string_test);
    let is_success = test1 == "atlam".to_string();
    println!("Working with strings in Rust!");
    println!("Reverse strings in Rust: success?: {:?}", is_success);
}
