fn main() {
    let name = "Rust";
    println!("Hello, {}!", name);
    println!(
        "por padrão as variaveis são imutaveis (constantes)\n
    para tornar uma variavel mutavel é necessario usar o modificador mut "
    );
    let mut mutable_variable = "Rust";
    println!("Hello, {}!", mutable_variable);
    mutable_variable = "Rust 2";
    println!("Hello, {}!", mutable_variable);
}
