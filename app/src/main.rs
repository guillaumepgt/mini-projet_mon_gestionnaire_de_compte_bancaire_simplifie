fn main() {
    let mut balance = 1000;

    start(balance)
}

fn start(balance: i32) {
    println!("Welcome in your bank account");
    println!("1. Check balance");
    println!("2. Add money");
    println!("3. Withdraw money");
    println!("4. Exit");

    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    if input.trim() == "1" {
        println!("Your balance is: {}", balance);
        start(balance)
    } else if input.trim() == "2" {
        add_money(balance);
    } else if input.trim() == "3" {
        withdraw_money(balance);
    } else if input.trim() == "4" {
        println!("Goodbye");
    } else {
        println!("Invalid input");

    }
}

fn add_money(balance: i32) {
    println!("Enter the amount you want to add");
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let balance = balance + input.trim().parse::<i32>().unwrap();
    println!("Your balance is: {}", balance);
    start(balance)
}

fn withdraw_money(balance : i32) {
    println!("Enter the amount you want to withdraw");
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let balance = balance - input.trim().parse::<i32>().unwrap();
    println!("Your balance is: {}", balance);
    start(balance)
}