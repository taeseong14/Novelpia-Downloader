use std::io;

fn main() {
    println!("Novelpia Downloader with Rust by taeseong14\n\n[Login]\n");
    println!("id: ");
    let mut id = String::new();
    io::stdin().read_line(&mut id).expect("Failed to read line");
    // if id doesn't contain @, add @gmail.com
    if !id.contains("@") {
        id = id.trim().to_string() + "@gmail.com";
        println!("{}", id);
    }
    println!("pw: ");
    let mut pw = String::new();
    io::stdin().read_line(&mut pw).expect("Failed to read line");
}
