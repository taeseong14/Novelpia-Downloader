use reqwest;
use serde_json::Value;
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    println!("Novelpia Downloader with Rust by taeseong14\n\n[Login]\n");
    print!("id: ");
    let mut id = String::new();
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

    let client = reqwest::Client::new();
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

    let loginkey = json["result"].as_str().unwrap();
    println!("Login success: {}", loginkey);
}
