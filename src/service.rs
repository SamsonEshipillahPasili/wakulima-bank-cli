use std::{collections::HashMap, io::stdin};

use crate::models::{Account, Bank};

impl Account {
    fn new(id: String, opening_balance: u32) -> Self {
        Self {
            id,
            opening_balance,
            current_balance: opening_balance,
        }
    }
}

impl Bank {
    pub fn init() -> Self {
        Self {
            accounts: HashMap::new(),
        }
    }

    pub fn list_accounts(&self) {
        println!("\n---- Accounts ----");
        for (idx, key) in self.accounts.keys().enumerate() {
            println!("{} {}", idx + 1, key)
        }
    }

    fn prompt(prompt: &str) -> Option<String> {
        println!("{}", prompt);
        let mut buffer = String::new();
        let read_result = stdin().read_line(&mut buffer);

        if let Err(error) = read_result {
            println!("Input error: {error:?}");
            return None;
        }

        Some(buffer)
    }

    pub fn open_account(&mut self) {
        println!("\n---- Open Account ----");
        let Some(account_id) = Self::prompt("Enter the account id:") else {
            return;
        };

        if self.accounts.contains_key(&account_id) {
            eprintln!("Account name is taken");
            return;
        }

        let Some(opening_balance) = Self::prompt("Enter the opening balance:") else {
            return;
        };

        let Ok(opening_balance) = opening_balance.trim().parse::<u32>() else {
            eprintln!("Invalid opening balance!");
            return;
        };

        let account = Account::new(account_id.clone(), opening_balance);
        self.accounts.insert(account_id, account);

        println!("The account was opened successfully!");
    }

    pub fn deposit(&mut self) {
        let Some(account_id) = Self::prompt("Enter the account id") else {
            return;
        };

        let Some(account) = self.accounts.get_mut(&account_id) else {
            eprintln!("No such account!");
            return;
        };

        let Some(amount) = Self::prompt("Enter the amount to deposit") else {
            return;
        };

        let Ok(amount) = amount.trim().parse::<u32>() else {
            eprintln!("Invalid amount!");
            return;
        };

        account.current_balance += amount;
    }
}
