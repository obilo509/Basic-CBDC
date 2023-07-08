use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref MAX_SUPPLY: Mutex<u64> = Mutex::new(10000000000);
    static ref ACCESSED_COIN: Mutex<u64> = Mutex::new(0);
    static ref WITHDRAW_COIN: Mutex<u64> = Mutex::new(0);
    static ref MAX_ACCT_LEN: u64 = 10;
}

struct UserDetails {
    user_name: String,
    balances: HashMap<String, u64>,
}

impl UserDetails {
    fn new() -> Self {
        UserDetails {
            user_name: String::new(),
            balances: HashMap::new(),
        }
    }

    fn create_account(&mut self, account_id: String, acct_name: &str) {
        if account_id.len() != *MAX_ACCT_LEN as usize {
            println!("Account Digit must be {}", *MAX_ACCT_LEN);
        } else {
            self.balances.insert(account_id, 0);
            self.user_name.push_str(acct_name);
        }
    }

    fn deposit(&mut self, account_id: &str, amount: u64) {
        if let Some(balance) = self.balances.get_mut(account_id) {
            let mut max_supply = MAX_SUPPLY.lock().unwrap();
            let mut accessed_coin = ACCESSED_COIN.lock().unwrap();

            *max_supply -= amount;
            *accessed_coin += amount;
            *balance += amount;
        }
    }

    fn withdraw(&mut self, account_id: &str, amount: u64) -> Result<(), String> {
        if let Some(balance) = self.balances.get_mut(account_id) {
            if *balance >= amount {
                let mut withdraw_coin = WITHDRAW_COIN.lock().unwrap();
                let mut accessed_coin = ACCESSED_COIN.lock().unwrap();

                *withdraw_coin += amount;
                *accessed_coin -= amount;
                *balance -= amount;

                Ok(())
            } else {
                Err(String::from("Insufficient Balance"))
            }
        } else {
            Err(String::from("Account not found"))
        }
    }

    fn get_balance(&self, account_id: &str) -> Option<u64> {
        self.balances.get(account_id).cloned()
    }
}

fn main() {
    let mut cbdc = UserDetails::new();

    cbdc.create_account(String::from("Obilo"), "Obilo");
    cbdc.create_account(String::from("Christian"), "Christian");

    cbdc.deposit("Obilo", 10000);
    cbdc.deposit("Christian", 1000);

    if let Err(err) = cbdc.withdraw("Obilo", 70) {
        println!("Withdrawal error: {}", err);
    }

    if let Some(balance) = cbdc.get_balance("Christian") {
        println!("Christian's balance: {}", balance);
    }

    if let Some(balance) = cbdc.get_balance("Obilo") {
        println!("Obilo's balance: {}", balance);
    }
}
