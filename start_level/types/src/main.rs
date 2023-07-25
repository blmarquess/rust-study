fn main() {
    let ar: [i32; 10] = [1; 10];
    println!("Arrays : {:#?}", ar);
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod test_sum {
    use super::*;

    #[test]
    fn sum_test() {
        let expect_result = 3;
        let result = sum(1, 2);
        assert_eq!(expect_result, result);
    }
}
