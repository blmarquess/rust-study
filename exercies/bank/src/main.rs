mod bank;

use crate::bank::account::{Account, AccountType};

fn main() {
    let acc_joe = Account::new(
        "John Doe".to_string(),
        100.0,
        AccountType::Checking,
        "123456".to_string(),
    );
    let acc_jane = Account::new(
        "Jane Doe".to_string(),
        100.0,
        AccountType::Savings,
        "".to_string(),
    );

    println!("Esta é a conta do! {:?}", acc_joe);
    println!("Esta é a conta da! {:?}", acc_jane);

    let mut you_name: String = String::new();
    let mut you_password: String = String::new();

    println!("Digite seu nome: ");
    std::io::stdin()
        .read_line(&mut you_name)
        .expect("Falha ao ler o nome");
    println!("Digite seu deposito inicial: ");
    let mut input_money: String = String::new();
    std::io::stdin()
        .read_line(&mut input_money)
        .expect("Falha ao ler o saldo");
    let you_balance: f64 = input_money.trim().parse().expect("Falha ao ler o saldo");

    println!("Digite seu tipo de conta: ");
    let mut input_acc_type: String = String::new();
    std::io::stdin()
        .read_line(&mut input_acc_type)
        .expect("Falha ao ler o tipo de conta");

    let account_type: AccountType = match input_acc_type.trim().parse() {
        Ok(num) => match num {
            1 => AccountType::Checking,
            2 => AccountType::Savings,
            _ => panic!("Tipo de conta inválido"),
        },
        Err(_) => panic!("Tipo de conta inválido"),
    };

    println!("Digite sua senha: ");
    std::io::stdin()
        .read_line(&mut you_password)
        .expect("Falha ao ler a senha");

    let acc_you = Account::new(
        you_name.to_string(),
        you_balance,
        account_type,
        you_password.to_string(),
    );

    println!("Esta é a conta do {} {:?}", acc_you.name, acc_you);
}
