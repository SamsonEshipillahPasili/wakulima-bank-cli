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

    fn to_csv(&self) -> String {
        format!(
            "{},{},{}",
            self.id, self.opening_balance, self.current_balance
        )
    }

    fn from_csv(csv: &str) -> Option<Self> {
        let mut csv_values_iter = csv.split(",");
        let id = csv_values_iter.next()?;
        let opening_balance = csv_values_iter.next()?.parse::<u32>().ok()?;
        let current_balance = csv_values_iter.next()?.parse::<u32>().ok()?;

        Some(Self {
            id: id.into(),
            opening_balance,
            current_balance,
        })
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

        for (idx, (key, value)) in self.accounts.iter().enumerate() {
            println!(
                "{}. Id: {}, Balance: {}",
                idx + 1,
                key,
                value.current_balance
            );
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

        Some(buffer.trim().into())
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

        println!("Deposit completed!");
    }

    pub fn withdraw(&mut self) {
        let Some(account) = Self::prompt("Enter the account name") else {
            return;
        };

        let Some(account) = self.accounts.get_mut(&account) else {
            eprintln!("No such account");
            return;
        };

        let Some(amount) = Self::prompt("Enter the amount") else {
            return;
        };

        let Ok(amount) = amount.trim().parse::<u32>() else {
            eprintln!("Invalid amount!");
            return;
        };

        if amount > account.current_balance {
            eprintln!(
                "Amount: {} is greater than the balance: {}",
                amount, account.current_balance
            );
            return;
        }

        account.current_balance -= amount;

        println!("Withdrawal completed!");
    }

    pub fn close_account(&mut self) {
        let Some(account_id) = Self::prompt("Enter account id") else {
            return;
        };

        let Some(account) = self.accounts.remove(&account_id) else {
            eprintln!("No such account!");
            return;
        };

        println!("Account was removed successfully: {}!", account.id);
    }

    pub fn close(&self) {
        println!("Thanks for visiting us. Bye.");
    }
}
