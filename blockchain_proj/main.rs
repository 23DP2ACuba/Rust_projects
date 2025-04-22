#[macro_use]
extern crate serde_derive;

use std::io;
use std::process;
use std::io::Write;

mod blockchain;

fn input_str() -> String {
    io::stdout().flush().expect("failed to flush");
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("failde to read");
    inp.trim().to_string()
}

fn main() {
    let mut choice = String::new();

    print!("Enter your miner address: ");
    let miner_addr = input_str();
    print!("Enter a Difficulty: ");
    let difficulty = input_str();
    
    let diff = difficulty.trim().parse::<u32>().expect("failed to parse difficulty into integer");
    println!("generating genesis block!");
    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), diff);

    loop {   
        println!("Menu");
        println!("1) New Transaction");
        println!("2) Mine block");
        println!("3) Change Difficulty");
        println!("4) Change Reward");
        println!("0) Exit");
        print!("Enter your choice: ");
        io::stdout().flush().expect("nothing");
        choice = input_str();
        match choice.trim().parse().unwrap() {
            0 => {
                println!("exiting");
                process::exit(0);
            }
            1 =>{

                print!("Enter sender address: ");
                let sender = input_str();
                print!("Enter reciever address: ");
                let reciever = input_str();
                print!("Enter amount: ");
                let amount = input_str();

                let res = chain.new_transaction(sender.trim().to_string(), 
                                                    reciever.trim().to_string(),
                                                    amount.trim().parse().unwrap());
                match res {
                    true => println!("transaction added"),
                    false => println!("transaction failed"),
                }

            }   
            2 => {
                println!("Generating block");
                let res = chain.generate_new_block();
                match res {
                    true => println!("block generated"),
                    false => println!("block generation failed"),
                }
            }
            3 => {
                print!("Enter new difficulty:");
                let new_diff = input_str();
                let res = chain.update_difficulty(new_diff.trim().parse().unwrap()); 
                match res {
                    true => println!("Updated difficulty"),
                    false => println!("failed to update difficulty"),
                }
            }
            4 => {
                print!("Enter new reward: ");
                let new_reward = input_str();
                let res = chain.update_reward(new_reward.trim().parse().unwrap());
                match res {
                    true => println!("Updated reward"),
                    false => println!("failed to update reward"),
                }
            }
            _ => {
                println!("Invalid operation");
            }
        }
    }
}
