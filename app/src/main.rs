fn main() {
    println!("Welcome in your bank account");
    println!("1. Check balance");
    println!("2. Add money");

    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    if input.trim() == "1" {
        println!("Your balance is: {}", balance());
    } else if input.trim() == "2" {
        add_money();
    } else {
        println!("Invalid input");

    }
}

fn balance() -> i32 {
    let balance = 1000;
    balance
}

fn add_money() {
    println!("Enter the amount you want to add");
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let balance = balance() + input.trim().parse::<i32>().unwrap();
    println!("Your balance is: {}", balance);
}