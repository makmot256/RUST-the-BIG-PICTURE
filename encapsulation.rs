mod bank {
    pub struct BankAccount {
        // Private field: balance is encapsulated.
        balance: f64,
    }

    impl BankAccount {
        // Public constructor to initialize the account with an initial balance.
        pub fn new(initial_balance: f64) -> BankAccount {
            BankAccount { balance: initial_balance }
        }

        // Public getter for the balance.
        pub fn balance(&self) -> f64 {
            self.balance
        }
        // Public method to deposit funds.
        pub fn deposit(&mut self, amount: f64) {
            if amount > 0.0 {
                self.balance += amount;
                println!("Deposited {:.2}. New balance: {:.2}", amount, self.balance);
            } else {
                println!("Cannot deposit a non-positive amount.");
            }
        }
        // Public method to withdraw funds.
        pub fn withdraw(&mut self, amount: f64) -> Result<(), String> {
            if amount <= 0.0 {
                return Err(String::from("Withdrawal amount must be positive."));
            }
            if amount > self.balance {
                Err(String::from("Insufficient funds."))
            } else {
                self.balance -= amount;
                println!("Withdrew {:.2}. New balance: {:.2}", amount, self.balance);
                Ok(())
            }
        }
    }
}
fn main() {
    use bank::BankAccount;

    // Create a new bank account with an initial balance.
    let mut account = BankAccount::new(100.0);
    println!("Initial balance: {:.2}", account.balance());

    // Deposit funds.
    account.deposit(50.0);
    
    // Attempt a successful withdrawal.
    match account.withdraw(30.0) {
        Ok(_) => println!("Withdrawal successful."),
        Err(e) => println!("Withdrawal failed: {}", e),
    }

    // Attempt to withdraw more than the available balance.
    match account.withdraw(150.0) {
        Ok(_) => println!("Withdrawal successful."),
        Err(e) => println!("Withdrawal failed: {}", e),
    }
    println!("Final balance: {:.2}", account.balance());
}



