use futures::future;
use reqwest;
use serde_json::Value;
use std::{
    fs,
    io::{self, Read, Write},
};

#[tokio::main]
async fn main() {
    println!("Novelpia Downloader with Rust by taeseong14");
    println!("\n[Login]\n");

    let client = reqwest::Client::new();

    let mut id;
    let pw;

    // check the file exists: account.txt
    if fs::metadata("account.txt").is_ok() {
        // read the file
        let mut file = fs::File::open("account.txt").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let arr: Vec<&str> = contents.split("\n").collect();
        id = arr[0].to_string();
        pw = arr[1].to_string();
        println!("Login with {}...", id);
    } else {
        id = input("id: ");
        // if id doesn't contain @, add @gmail.com
        if !id.contains("@") {
            id = id.trim().to_string() + "@gmail.com";
            println!("id: {}", id);
        }
        pw = input("pw: ");
        println!();
        print!("\rLogin...");
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
        println!("\rLogin failed: {}", json["err"]);
        // remove the file
        fs::remove_file("account.txt").unwrap();
        println!("./account.txt removed");
        end();
        return;
    }

    // let loginkey = json["result"].as_str().unwrap();

    println!("\rLogin success\n");

    // write the file if not exists
    if fs::metadata("account.txt").is_err() {
        let mut file = fs::File::create("account.txt").unwrap();
        file.write_all(id.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
        file.write_all(pw.as_bytes()).unwrap();
        println!("Login data saved in ./account.txt\n");
    }

    let book_id = input("bookId: ");

    print!("\rloading book info...");

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

    let yn = input(format!("\r[{} - {}] is right? (y/n): ", book_name, book_author).as_str());
    // if yn doesn't contain y, exit
    if !yn.contains("y") {
        println!("exit");
        end();
        return;
    }

    print!("\rGet page.");

    let data = client
        .get("https://b-p.msub.kr/novelp/list/?p=all&id=".to_owned() + &book_id)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let json = serde_json::from_str::<Value>(&data).unwrap();
    let list = json["result"].as_array().unwrap();
    println!(
        "\rGet page. {}/{}\n",
        json["p"].as_u64().unwrap(),
        json["p"].as_u64().unwrap()
    );
    print!("\r0/0 [0%]");

    // string array
    let mut result = String::new();

    // make urls vector from list["link"]
    let urls = list
        .iter()
        .map(|x| "https://b-p.msub.kr/novelp/view/?id=".to_owned() + &x["link"].as_str().unwrap())
        .collect::<Vec<String>>();

    // for each of list make bodies with reqwest to htttp://b-p.msub.kr/novelp/view/?id=list[i]["link"]
    let bodies = future::join_all(urls.into_iter().map(|url| {
        let client = &client;
        async move {
            let resp = client.get(url).send().await?;
            resp.text().await
        }
    }))
    .await;

    let mut i = 0;
    for b in bodies {
        match b {
            Ok(b) => {
                let json = serde_json::from_str::<Value>(&b).expect("json parse error");
                let content = json["result"].as_str().unwrap();
                result += list[i]["title"].as_str().unwrap();
                i += 1;
                result += "\n\n\n\n\n";
                result += content;
                result += "\n\n\n\n\n\n\n\n\n\n";
                print!("\r{}/{} [{}%]", i, list.len(), i * 100 / list.len());
            }
            Err(e) => eprintln!("Got an error: {}", e),
        }
    }

    // save result in ./result.txt
    let mut file = fs::File::create("result.txt").unwrap();
    file.write_all(result.as_bytes()).unwrap();
    println!("\n\nDone! check ./result.txt");

    end();
}

// make function input with str type String
fn input(msg: &str) -> String {
    let mut input = String::new();
    print!("{}", msg);
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut input).unwrap();
    input
}

// print press enter to exit
fn end() {
    println!("\n\nPress enter to exit.");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}
