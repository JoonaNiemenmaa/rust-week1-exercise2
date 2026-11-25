use std::io;

fn main() {
    let mut buf = String::new();

    io::stdin()
        .read_line(&mut buf)
        .expect("Virheellinen syöte.");

    let syöte = buf.trim().to_ascii_lowercase();

    match syöte.as_str() {
        "rust" => println!("So you appreciate Rust? That's great! Thank you!"),
        "no" => println!("So you appreciate Rust? That's great! Thank you!"),
        _ => println!("It seems that you like {syöte}."),
    }
}
