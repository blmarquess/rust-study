#[derive(Debug)]
pub enum AccountType {
    Checking,
    Savings,
}
#[derive(Debug)]
pub struct Account {
    pub name: String,
    pub account_type: AccountType,
    pub balance: f64,
}

impl Account {
    pub fn new(name: String, balance: f64, a_type: AccountType) -> Account {
        Account {
            name,
            balance,
            account_type: a_type,
        }
    }

    pub fn deposit(&mut self, amount: f64) -> bool {
        if amount < 0.0 {
            println!("Cannot deposit negative amount");
            return false;
        }
        self.balance += amount;
        true
    }

    pub fn withdraw(&mut self, amount: f64) -> bool {
        if self.balance < 0.0 || self.balance < amount {
            println!("Cannot withdraw without balance");
            return false;
        }
        self.balance -= amount;
        true
    }

    pub fn transfer_to_account(&mut self, account: Account, value: f64) {
        if value > self.balance {
            print!(
                "Cannot has total valui in balance to done transfer. out {}",
                value - self.balance
            );
            return;
        }
    }
}

#[cfg(test)]
mod tests_for_account {
    const NAME: &str = "Marcio";
    const BALANCE: f64 = 100.0;
    const A_TYPE: super::AccountType = super::AccountType::Checking;

    fn sut() -> super::Account {
        super::Account::new(NAME.to_string(), BALANCE, A_TYPE)
    }

    #[test]
    fn should_deposit_if_positive_value() {
        let mut acc = sut();
        acc.deposit(100.0);
        assert_eq!(acc.balance, 200.0);
    }

    #[test]
    fn should_possible_withdraw_sucess() {
        let mut acc = sut();
        acc.withdraw(50.0);
        assert_eq!(acc.balance, 50.0);
    }

    #[test]
    fn should_not_withdraw_without_balance() {
        let mut acc = sut();
        acc.withdraw(200.0);
        assert_eq!(acc.balance, BALANCE);
    }

    #[test]
    fn should_not_deposit_if_negative_value() {
        let mut acc = sut();
        acc.deposit(-10.00);
        assert_eq!(acc.balance, BALANCE);
    }
}

#[cfg(test)]
mod testes {
    #[test]
    fn array_iteration() {
        let arr: [u8; 3] = [3, 2, 1];
        let mut iterator = arr.iter();
        assert!(iterator.next().unwrap() == &3);
        assert!(iterator.next().unwrap() == &2);
        assert!(iterator.next().unwrap() == &1);
    }
}
