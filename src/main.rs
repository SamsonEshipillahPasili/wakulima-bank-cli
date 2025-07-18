pub mod models;
pub mod service;

use std::io::stdin;

fn display_main_menu() -> models::MainMenuUserOptions {
    let prompt = r#"
    Welcome to Mukulima Bank
    --------------------------
    Please Select your options

    1. List Bank Accounts
    2. Open Bank Account
    3. Deposit
    4. Withdraw
    5. Close Bank Account
    6. Exit

    Please your selection:
    "#;

    println!("{prompt}");

    let mut buff = String::new();
    let read_result = stdin().read_line(&mut buff);
    if let Err(error) = read_result {
        eprintln!("Could not read user input: {:?}", error);
        return models::MainMenuUserOptions::InputError;
    };

    let Ok(user_selection) = buff.trim().parse::<u8>() else {
        return models::MainMenuUserOptions::InvalidInput;
    };

    match user_selection {
        1 => models::MainMenuUserOptions::ListBankAccounts,
        2 => models::MainMenuUserOptions::OpenbankAccount,
        3 => models::MainMenuUserOptions::Deposit,
        4 => models::MainMenuUserOptions::Withdraw,
        5 => models::MainMenuUserOptions::CloseAccount,
        6 => models::MainMenuUserOptions::Exit,
        _ => models::MainMenuUserOptions::InvalidInput,
    }
}

fn main() {
    let Some(mut bank) = models::Bank::init() else {
        eprintln!("Failed to init bank!");
        return;
    };

    loop {
        let selection = display_main_menu();

        match selection {
            models::MainMenuUserOptions::ListBankAccounts => bank.list_accounts(),
            models::MainMenuUserOptions::OpenbankAccount => bank.open_account(),
            models::MainMenuUserOptions::Deposit => bank.deposit(),
            models::MainMenuUserOptions::Withdraw => bank.withdraw(),
            models::MainMenuUserOptions::CloseAccount => bank.close_account(),
            models::MainMenuUserOptions::Exit => {
                bank.close();
                break;
            }
            models::MainMenuUserOptions::InputError => {
                eprintln!("There was an error reading input.");
                break;
            }
            models::MainMenuUserOptions::InvalidInput => {
                eprintln!("Invalid selection. Please try again!");
            }
        }
    }
}
