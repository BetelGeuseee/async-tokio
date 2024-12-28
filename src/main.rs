use std::future::Future;
use std::time::Duration;
use mini_redis::{client, Result};
use tokio::time;

#[tokio::main]
async fn main() {
    let join_handle_task1 = tokio::spawn(async {
        println!("Performing hello task1");
        time::sleep(Duration::from_secs(5)).await;
        return "Hello Task1";
    });
    println!("first print");
    let output = join_handle_task1.await.unwrap();
    println!("{}", output);
    println!("first print");
    let join_handle_task2 = tokio::spawn(async {
        time::sleep(Duration::from_secs(5)).await;
        return "Hello Task2";
    });
    let output = join_handle_task2.await.unwrap();
    println!("{}", output);

    println!("second print");
}

//the return value of async fn is the anonymous type that returns the Future trait
async fn task1() {
    time::sleep(Duration::from_secs(5)).await;
    println!("Inside task 1");
}

async fn task2() {
    time::sleep(Duration::from_secs(5)).await;
    println!("Inside task 2");
}
