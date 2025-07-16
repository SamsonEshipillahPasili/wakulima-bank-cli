use std::{collections::HashMap, io::stdin};

#[derive(Debug)]
enum MainMenuUserOptions {
    ListBankAccounts,
    OpenbankAccount,
    Deposit,
    Withdraw,
    Exit,
    InputError,
    InvalidInput,
}

struct Account {
    id: String,
    opening_balance: u32,
    current_balance: u32,
}

impl Account {
    fn new(id: String, opening_balance: u32) -> Self {
        Self {
            id,
            opening_balance,
            current_balance: opening_balance,
        }
    }
}

struct Bank {
    accounts: HashMap<String, Account>,
}

impl Bank {
    fn init() -> Self {
        Self {
            accounts: HashMap::new(),
        }
    }

    fn list_accounts(&self) {
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

    fn open_account(&mut self) {
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
}

fn display_main_menu() -> MainMenuUserOptions {
    let prompt = r#"
    Welcome to Mukulima Bank
    --------------------------
    Please Select your options

    1. List Bank Accounts
    2. Open Bank Account
    3. Deposit
    4. Withdraw
    5. Exit

    Please your selection:
    "#;

    println!("{prompt}");

    let mut buff = String::new();
    let read_result = stdin().read_line(&mut buff);
    if let Err(error) = read_result {
        eprintln!("Could not read user input: {:?}", error);
        return MainMenuUserOptions::InputError;
    };

    let Ok(user_selection) = buff.trim().parse::<u8>() else {
        return MainMenuUserOptions::InvalidInput;
    };

    match user_selection {
        1 => MainMenuUserOptions::ListBankAccounts,
        2 => MainMenuUserOptions::OpenbankAccount,
        3 => MainMenuUserOptions::Deposit,
        4 => MainMenuUserOptions::Withdraw,
        5 => MainMenuUserOptions::Exit,
        _ => MainMenuUserOptions::InvalidInput,
    }
}

fn main() {
    let mut bank = Bank::init();

    let selection = display_main_menu();

    println!("{selection:?}");

    match selection {
        MainMenuUserOptions::ListBankAccounts => bank.list_accounts(),
        MainMenuUserOptions::OpenbankAccount => bank.open_account(),
        MainMenuUserOptions::Deposit => todo!(),
        MainMenuUserOptions::Withdraw => todo!(),
        MainMenuUserOptions::Exit => todo!(),
        MainMenuUserOptions::InputError => todo!(),
        MainMenuUserOptions::InvalidInput => todo!(),
    }
}
