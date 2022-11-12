use reqwest;
use serde_json::Value;
use std::{
    fs,
    io::{self, Read, Write},
};

#[tokio::main]
async fn main() {
    println!("Novelpia Downloader with Rust by taeseong14\n\n[Login]\n");

    let client = reqwest::Client::new();

    let mut id = String::new();
    let mut pw = String::new();

    // check the file exists: account.txt
    if fs::metadata("account.txt").is_ok() {
        // read the file
        let mut file = fs::File::open("account.txt").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let arr: Vec<&str> = contents.split("\n").collect();
        id = arr[0].to_string();
        pw = arr[1].to_string();
        println!("login with {}...", id);
    } else {
        print!("id: ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut id).unwrap();
        // if id doesn't contain @, add @gmail.com
        if !id.contains("@") {
            id = id.trim().to_string() + "@gmail.com";
            println!("id: {}", id);
        }
        print!("pw: ");
        let mut pw = String::new();
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut pw).expect("Failed to read line");

        println!("\nlogin...");
    }
    let data = client
        .post("https://b-p.msub.kr/novelp/login")
        // body with json
        .json(&serde_json::json!({
            "id": id,
            "pw": pw,
        }))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let json = serde_json::from_str::<Value>(&data).unwrap();
    // check if json["err"] is null
    if !json["err"].is_null() {
        println!("Login failed: {}", json["err"]);
        return;
    }

    // let loginkey = json["result"].as_str().unwrap();

    println!("login success");

    // write the file if not exists
    if fs::metadata("account.txt").is_err() {
        let mut file = fs::File::create("account.txt").unwrap();
        file.write_all(id.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
        file.write_all(pw.as_bytes()).unwrap();
        println!("login data saved in ./account.txt");
    }

    print!("bookId: ");
    let mut book_id = String::new();
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut book_id).unwrap();

    println!("\rloading book info...");

    let data = client
        .get("https://b-p.msub.kr/novelp/info/?id=".to_owned() + &book_id)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let json = serde_json::from_str::<Value>(&data).unwrap();
    let book_name = json["result"]["title"].as_str().unwrap();
    let book_author = json["result"]["author"].as_str().unwrap();
    print!("[{} - {}] is right? (y/n): ", book_name, book_author);
    let mut yn = String::new();
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut yn).unwrap();
    // if yn doesn't contain y, exit
    if !yn.contains("y") {
        println!("exit");
        return;
    }

    println!("Get page.");

    let data = client
        .get("https://b-p.msub.kr/novelp/list/?p=all&id=".to_owned() + &book_id)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("{}", data);
    let json = serde_json::from_str::<Value>(&data).unwrap();
    let result = json["result"].as_array().unwrap();
    println!("{}", result.len());
}
