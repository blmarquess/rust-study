// Introduction
// Reversing strings (reading them from right to left, rather than from left to right) is a surprisingly common task in programming.
// For example, in bioinformatics, reversing the sequence of DNA or RNA strings is often important for various analyses, such as finding complementary strands or identifying palindromic sequences that have biological significance.
// Instructions
// Your task is to reverse a given string.
// Some examples:
// Turn "stressed" into "desserts".
// Turn "strops" into "sports".
// Turn "racecar" into "racecar".
pub fn reverse_string_native_method(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn reverse_string_for_char(s: &str) -> String {
    let mut result: Vec::<char> = s.chars().collect();
    let mut i = 0;
    let mut j = result.len() - 1;

    while i < j {
        result.swap(i, j);
        i += 1;
        j -= 1;
    }
    result.into_iter().collect()
}


#[cfg(test)]
mod tests_reverse_string {
    use crate::reverse_string::{reverse_string_for_char, reverse_string_native_method};

    #[test]
    fn reverse_string_native_method_test() {
        assert_eq!(reverse_string_native_method("stressed"), "desserts");
        assert_eq!(reverse_string_native_method("strops"), "sports");
    }

    #[test]
    fn reverse_string_for_char_test() {
        assert_eq!(reverse_string_for_char("stressed"), "desserts");
        assert_eq!(reverse_string_for_char("strops"), "sports");
    }
}
