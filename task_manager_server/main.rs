mod models;
mod services;
use std::io;
use models::tasks_model::{Task};
use services::services::{
    create_task,
    get_tasks,
    update_task,
    complete_task,
    delete_task
};

use mongodb::{
    options::ClientOptions,
    Client, error::Result as MongoResult
};

fn read_inp() -> String {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("Failed to read input");
    inp.trim().to_lowercase().to_string()
}

#[tokio::main] 
async fn main() -> MongoResult<()>{
    let uri = "mongodb+srv://nggrK1ller:Mybf2C9ZvSchyDC@taskdb.4iat1gp.mongodb.net/?retryWrites=true&w=majority&appName=taskDB";
    let client_options = ClientOptions::parse(uri).await?;
    let client = Client::with_options(client_options)?;
    let db = client.database("taskDB");
    let  collection = db.collection::<Task>("tasks");
    let mut quit = false;
    while !quit {
        println!("\ncommands >> 'create', 'get', 'update', 'complete', 'delete' or 'quit'");
        let inp = read_inp();
        match inp.as_str() {
            "create" => {
                println!("Enter task title:");
                let title = read_inp();
                println!("Enter task description:");
                let description = read_inp();
                create_task(&collection, &title, &description).await.unwrap();
            },
            "get" => {
                println!("Fetching Tasks...");
                get_tasks(&collection).await.expect("Failed getting tasks");
            },
            "update" => {
                println!("Enter title:");
                let new_title = read_inp();
                println!("Enter new description:");
                let new_desc = read_inp();
                update_task(&collection, new_title.as_str(), new_desc.as_str()).await.expect("failed updating task");

            },
            "complete" => {
                println!("Enter title:");
                let title = read_inp();
                complete_task(&collection, title.as_str()).await.expect("failed completing");

            },
            "delete" => {
                println!("Enter title:");
                let title = read_inp();
                delete_task(&collection, title.as_str()).await.expect("failed completing");

            }
            "quit" => {
                println!("Exiting...");
                quit = true;
            },
            _ => {
                println!("❌ Unknown command. Please type 'create', 'get', 'update', 'complete', 'delete' or 'quit'.");
            }
        }

        
    }
    Ok(())
}
