fn main() {
    let name = "Rust";
    println!("Hello, {}!", name);
    println!(
        "por padrão as variaveis são imutaveis (constantes)\n
        para tornar uma variavel mutavel é necessario usar o modificador mut "
    );
    let mut mutableVariable = "Rust";
    println!("Hello, {}!", mutableVariable);
    mutableVariable = "Rust 2";
    println!("Hello, {}!", mutableVariable);
}
