use chrono::Local;

#[derive(Debug)]
pub enum AccountType {
    Checking,
    Savings,
}

#[derive(Debug, PartialEq)]
pub enum TransactionType {
    Deposit,
    Withdraw,
    Transfer,
}

#[derive(Debug)]
pub struct Transaction {
    ammount: f64,
    date: String,
    transaction_type: TransactionType,
}

impl Transaction {
    pub fn new(ammount: f64, transaction_type: TransactionType) -> Transaction {
        if ammount <= 0.0 {
            panic!("Cannot possible to create an account with a negative or zero balance");
        }

        Transaction {
            ammount,
            date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            transaction_type,
        }
    }
}

#[derive(Debug)]
pub struct Account {
    pub name: String,
    pub account_type: AccountType,
    pub balance: f64,
    pub transaction_history: Vec<Transaction>,
}

impl Account {
    pub fn new(name: String, balance: f64, a_type: AccountType) -> Account {
        let transaction = Transaction::new(balance, TransactionType::Deposit);
        let mut transaction_history: Vec<Transaction> = Vec::new();
        transaction_history.push(transaction);
        Account {
            name,
            balance,
            account_type: a_type,
            transaction_history,
        }
    }

    fn add_transaction(&mut self, amount: f64, transaction_type: TransactionType) {
        let transaction = Transaction::new(amount, transaction_type);
        self.transaction_history.push(transaction);
    }

    pub fn deposit(&mut self, amount: f64) -> bool {
        if amount < 0.0 {
            println!("Cannot deposit negative amount");
            return false;
        }
        self.balance += amount;
        self.add_transaction(amount, TransactionType::Deposit);
        true
    }

    pub fn withdraw(&mut self, amount: f64) -> bool {
        if self.balance < 0.0 || self.balance < amount {
            println!("Cannot withdraw without balance");
            return false;
        }
        self.balance -= amount;
        self.add_transaction(amount, TransactionType::Withdraw);
        true
    }

    pub fn transfer_to_account(&mut self, account: &mut Account, amount: f64) {
        if amount > self.balance {
            print!(
                "Cannot has total valui in balance to done transfer. out {}",
                amount - self.balance
            );
            return;
        }
        self.withdraw(amount);
        self.add_transaction(amount, TransactionType::Transfer);
        account.deposit(amount);
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

    #[test]
    fn shold_possible_transfer_betwaeen_accounts() {
        let mut acc_1 = sut();
        let mut acc_2 = sut();
        acc_1.transfer_to_account(&mut acc_2, 58.0);
        assert_eq!(acc_1.balance, 42.0);
        assert_eq!(acc_2.balance, 158.0);
    }

    #[test]
    fn shold_not_possible_transfer_betwaeen_accounts_without_balance() {
        let mut acc_1 = sut();
        let mut acc_2 = sut();
        acc_1.transfer_to_account(&mut acc_2, 200.0);
        assert_eq!(acc_1.balance, 100.0);
        assert_eq!(acc_2.balance, 100.0);
    }

    #[test]
    fn shold_not_possible_transfer_betwaeen_accounts_without_balance_in_account() {
        let mut acc_1 = sut();
        let mut acc_2 = sut();
        acc_1.withdraw(100.0);
        acc_1.transfer_to_account(&mut acc_2, 200.0);
        assert_eq!(acc_1.balance, 0.0);
        assert_eq!(acc_2.balance, 100.0);
    }

    #[test]
    fn shold_has_transaction_history() {
        let mut acc = sut();
        acc.deposit(100.0);
        acc.withdraw(50.0);
        assert_eq!(acc.transaction_history.len(), 3);
    }

    #[test]
    fn shold_has_transaction_history_with_correct_values() {
        let mut acc = sut();
        acc.deposit(100.0);
        acc.withdraw(50.0);
        acc.deposit(50.0);
        assert_eq!(acc.transaction_history[1].ammount, 100.0);
        assert_eq!(acc.transaction_history[2].ammount, 50.0);
        assert_eq!(acc.transaction_history[3].ammount, 50.0);
    }

    #[test]
    fn shold_has_transaction_history_with_correct_transaction_type() {
        let mut acc = sut();
        acc.deposit(100.0);
        acc.withdraw(50.0);
        acc.deposit(50.0);
        assert_eq!(
            acc.transaction_history[1].transaction_type,
            super::TransactionType::Deposit
        );
        assert_eq!(
            acc.transaction_history[2].transaction_type,
            super::TransactionType::Withdraw
        );
        assert_eq!(
            acc.transaction_history[3].transaction_type,
            super::TransactionType::Deposit
        );
    }
}
