fn main() {
    let collection_prime = collect_prime_with_loop(100);
    println!(
        "os seguintes números são primos de 0 até 100, \n {:?}",
        collection_prime
    );
}

fn is_prime(num: i64) -> bool {
    let res;
    for i in 2..num {
        if num % i == 0 {
            return false;
        }
    }
    if num > 1 && num != 1 {
        res = true
    } else {
        res = false
    };
    res
}

pub fn collect_prime_with_loop(range: i64) -> Vec<i64> {
    let mut collection_prime: Vec<i64> = Vec::new();
    for num in 0..range {
        if is_prime(num) {
            collection_prime.push(num);
        }
    }
    collection_prime
}

#[cfg(test)]
mod test_loop_study {
    #[test]
    fn test_collect_num_prime_at_range_ok() {
        let vector_prim = super::collect_prime_with_loop(10);
        println!("{:?}", vector_prim);
        assert_eq!(vector_prim.len(), 4)
    }

    #[test]
    fn test_collect_num_prime_at_range_fail() {
        let vector_prim = super::collect_prime_with_loop(10);
        println!("{:?}", vector_prim);
        assert_ne!(vector_prim.len(), 2)
    }
}
