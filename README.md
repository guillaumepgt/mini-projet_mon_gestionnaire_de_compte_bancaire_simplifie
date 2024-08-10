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
   <code-block lang="bash">git checkout -b create_dockerfile</code-block>   
   <code-block lang="bash">git push origin create_dockerfile</code-block>
8. Create a dockerfile to run the project.
   <code-block lang="docker">FROM rust:latest
   WORKDIR /myapp
   COPY . .
   RUN cargo build --release && cargo run
   CMD ["./target/release/app"]</code-block>
9. Run dockerfile.
   <code-block lang="bash">docker build -t myapp .</code-block>
   <code-block lang="bash">docker run --rm -it myapp</code-block>
10. Create a struct to represent a bank account.
11. Implement methods to deposit and withdraw money.
12. Implement a method to display the account balance.
13. execute the program.
    <code-block lang="bash">cargo build -t app . && cargo run --rm -it <app></app></code-block>
    
