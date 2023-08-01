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


#[cfg(test)]
mod atm_tests {
    sut() -> super::AutomatedTellerMachine {
        super::AutomatedTellerMachine::new()
    }
}