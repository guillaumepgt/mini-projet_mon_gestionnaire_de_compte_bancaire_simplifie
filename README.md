# mini-project my bank account manager simplifies

## objective
I want to create a simple bank account manager that allows me to keep track of my expenses and income.
And learn how to use Rust.

![Rust](img/Rust.png)

## Step by step
1. Create a repository on GitHub.
2. Create a folder for the project.
   ```bash
   mkdir mini-project_my_bank_account_manager_simplifies
   ```
4. Create a new Rust project.
   ```bash
   Cargo new app```
6. Link Git and GitHub.
   ```bash
   git init
   git remote add origin
   git add .
   git commit -m "Initial commit"
   git push -u origin main ```
7. Create a new branch.
   ```bash
   git checkout -b create_dockerfile
   git push origin create_dockerfile```
8. Create a dockerfile to run the project.
   ```docker
   FROM rust:latest
   WORKDIR /myapp
   COPY . .
   RUN cargo build --release && cargo run
   CMD ["./target/release/app"]```
9. Run dockerfile.
   ```bash
   docker build -t myapp .
   docker run --rm -it myapp```
10. Create a struct to represent a bank account.
11. Implement methods to deposit and withdraw money.
12. Implement a method to display the account balance.
    
