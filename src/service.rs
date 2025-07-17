use std::fs;
use std::path::PathBuf;
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
    pub fn init() -> Option<Self> {
        let path = PathBuf::from("data.csv");
        if !path.exists() {
            return Some(Self {
                accounts: HashMap::new(),
            });
        }

        let file_read_result = fs::read_to_string(path);
        let file_content = match file_read_result {
            Ok(content) => content,
            Err(err) => {
                eprintln!("Error reading file: {err:?}");
                return None;
            }
        };

        let mut accounts = HashMap::new();

        for line in file_content.split("\n") {
            let Some(account) = Account::from_csv(line) else {
                continue;
            };

            accounts.insert(account.id.clone(), account);
        }

        Some(Self { accounts })
    }

    fn perist(&self) {
        let mut data = String::new();
        for value in self.accounts.values() {
            let line = format!("{}\n", value.to_csv());
            data.push_str(&line);
        }

        if let Err(e) = fs::write("data.csv", data) {
            eprintln!("There was an error: {e:?}");
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

        self.perist();

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

        self.perist();

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

        self.perist();

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

        self.perist();

        println!("Account was removed successfully: {}!", account.id);
    }

    pub fn close(&self) {
        println!("Thanks for visiting us. Bye.");
    }
}
