mod bank;

use crate::bank::account::{Account, AccountType};

fn main() {
    let acc_joe = Account::new("John Doe".to_string(), 100.0, AccountType::Checking);
    let acc_jane = Account::new("Jane Doe".to_string(), 100.0, AccountType::Savings);

    println!("Esta é a conta do! {:?}", acc_joe);
    println!("Esta é a conta da! {:?}", acc_jane);
}
