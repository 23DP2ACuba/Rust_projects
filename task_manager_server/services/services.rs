use crate::models::tasks_model::{Task, Status};
use futures_util::stream::TryStreamExt;
use std::error::Error;
use mongodb::{
    Collection,
    bson::doc,
};


pub async fn create_task(collection: &Collection<Task>, title: &str, description: &str) -> Result<(), Box<dyn Error>> {
    let new_task = Task {
        id: None,
        title: title.to_string(),
        description: description.to_string(),
        status: Status::NotCompleted,
    };

    collection.insert_one(new_task, None).await?;
    println!("Task created");
    Ok(())
}

pub async fn get_tasks(collection: &Collection<Task>) -> Result<(), Box<dyn Error>> {
    let mut cursor = collection.find(None, None).await?;
    while let Some(task) = cursor.try_next().await? {
        println!("Task: {:?}", task);
    }
    
    Ok(())
}

pub async fn update_task(collection: &Collection<Task>, new_title: &str, new_desc: &str,) -> Result<(), Box<dyn Error>> {
    let filter = doc! {
        "title": new_title,
    };
    let update = doc! {
        "$set": {
            "description": new_desc,
            "status": "not completed",
        }
    };

    let res = collection.update_one(filter, update, None).await?;
    println!("Updated {} documents", res.modified_count);
    Ok(())
}


pub async fn complete_task(collection: &Collection<Task>, title: &str) -> Result<(), Box<dyn Error>> {
    let filter = doc! {
        "title": title,
    };
    let update = doc! {
        "$set": {
            "status": "completed",
        }
    };
    let res = collection.update_one(filter, update, None).await?;
    println!("Completed task: {}", title);
    Ok(())
}

pub async fn delete_task(collection: &Collection<Task>, title: &str) -> Result<(), Box<dyn Error>> {
    let filter = doc! {
        "title": title,
    };
    let res = collection.delete_one(filter, None).await?;
    println!("Deleted task: {}", title);
    Ok(())
}
