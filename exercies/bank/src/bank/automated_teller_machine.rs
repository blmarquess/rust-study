use uuid::Uuid;

use super::account::{Account, AccountType, Transaction};

pub enum operation {
    Deposit,
    Withdraw,
    Transfer,
    CreateAccount,
    CheckBalance,
    CheckTransactionHistory,
}

pub struct AutomatedTellerMachine {
    pub terminal_id: String,
    pub accounts: Vec<Account>,
    pub transaction_history: Vec<Transaction>,
}

impl AutomatedTellerMachine {
    pub fn new() -> AutomatedTellerMachine {
        AutomatedTellerMachine {
            terminal_id: Uuid::new_v4().to_string(),
            accounts: Vec::new(),
            transaction_history: Vec::new(),
        }
    }

    pub fn create_account(
        &mut self,
        name: String,
        balance: f64,
        account_type: AccountType,
        password: String,
    ) -> bool {
        let account = Account::new(name, balance, account_type, password);
        self.accounts.push(account);
        true
    }
}

#[cfg(test)]
mod atm_tests {
    fn sut() -> super::AutomatedTellerMachine {
        super::AutomatedTellerMachine::new()
    }
}
