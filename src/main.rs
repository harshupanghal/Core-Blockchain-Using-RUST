use std::io;
use std::io::Write;
use std::process;

mod blockchain;

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); 
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input\n");
    input.trim().to_string()
}

fn main() {
    let miner_addr = get_input("Input a miner address: ");
    let difficulty = get_input("Difficulty: ");
    let diff = difficulty.trim().parse::<u32>().expect("We need an integer");
    
    println!("Generating genesis block! ");
    let mut chain = blockchain::Chain::new(miner_addr, diff);

    loop {
        println!("Menu");
        println!("1) New Transaction");
        println!("2) Mine block");
        println!("3) Change Difficulty");
        println!("4) Change Reward");
        println!("0) Exit");
        
        let choice = get_input("Enter your choice: ");
        println!("");

        match choice.trim().parse::<u32>().unwrap_or_else(|_| {
            println!("Invalid choice. Please enter a number.\n");
            return 9999;
        }) {
            0 => {
                println!("Exiting!");
                process::exit(0);
            }
            1 => {
                let sender = get_input("Enter sender address: ");
                let receiver = get_input("Enter receiver address: ");
                let amount = get_input("Enter amount: ");
                
                let res = chain.new_transaction(
                    sender,
                    receiver,
                    amount.trim().parse().expect("Amount should be a number\n"),
                );

                match res {
                    true => println!("Transaction added\n"),
                    false => println!("Transaction failed\n"),
                }
            }
            2 => {
                println!("Generating block\n");
                let res = chain.generate_new_block();
                match res {
                    true => println!("Block generated successfully\n"),
                    false => println!("Block generation failed\n"),
                }
            }
            3 => {
                let new_diff = get_input("Enter new difficulty: ");
                let res = chain.update_difficulty(new_diff.trim().parse().unwrap());
                match res {
                    true => println!("Updated difficulty\n"),
                    false => println!("Failed to update difficulty\n"),
                }
            }
            4 => {
                let new_reward = get_input("Enter new reward: ");
                let res = chain.update_reward(new_reward.trim().parse().unwrap());
                match res {
                    true => println!("Updated reward\n"),
                    false => println!("Failed to update reward\n"),
                }
            }
            _ => println!("Invalid option, please retry.\n"),
        }
    }
}
