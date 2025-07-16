use std::collections::HashMap;

#[derive(Debug)]
pub enum MainMenuUserOptions {
    ListBankAccounts,
    OpenbankAccount,
    Deposit,
    Withdraw,
    Exit,
    InputError,
    InvalidInput,
}

pub struct Account {
    pub id: String,
    pub opening_balance: u32,
    pub current_balance: u32,
}

pub struct Bank {
    pub accounts: HashMap<String, Account>,
}
