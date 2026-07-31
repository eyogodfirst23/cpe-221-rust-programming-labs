#[derive(Debug, Clone)]
struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn new(owner: &str, initial_balance: f64) -> Self {
        BankAccount {
            owner: owner.to_string(),
            balance: initial_balance,
        }
    }

    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!("Deposited ${:.2}. New balance: ${:.2}", amount, self.balance);
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount > self.balance {
            return Err("Insufficient funds".into());
        }
        self.balance -= amount;
        Ok(())
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    let mut acc = BankAccount::new("Alice", 1000.0);
    acc.deposit(500.0);
    match acc.withdraw(200.0) {
        Ok(()) => println!("Withdrawal successful"),
        Err(msg) => println!("Error: {}", msg),
    }
    match acc.withdraw(999999.0) {
        Ok(()) => println!("Withdrawal successful"),
        Err(msg) => println!("Error: {}", msg),
    }
    println!("Final balance: ${:.2}", acc.balance());
}
