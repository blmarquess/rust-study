mod client;

use crate::{calc::sum, client::Cliente};

fn main() {
    let ar: [i32; 10] = [1; 10];
    println!("Arrays : {:#?}", ar);

    let idade = sum(1990, 5) as u16;

    let cliente = Cliente::new(String::from("Paulo"), idade, String::from("Onde?"));
    println!("Cliente Nome -> {}", cliente.nome);
    println!("Cliente idade -> {}", cliente.ano_de_nascimento);
}

mod calc {
    pub fn sum(a: u64, b: u64) -> u64 {
        a + b
    }
}

#[cfg(test)]
mod test_sum {
    use crate::calc::sum;

    #[test]
    fn sum_test() {
        let expect_result = 3;
        let result = sum(1, 2);
        assert_eq!(expect_result, result);
    }
}
