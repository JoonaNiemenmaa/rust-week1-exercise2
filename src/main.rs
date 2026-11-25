use std::io;

fn main() {
    let mut buf = String::new();

    io::stdin()
        .read_line(&mut buf)
        .expect("Virheellinen syöte.");

    let syöte = buf.trim();

    match syöte {
        "Rust" => println!("So you appreciate Rust? That's great! Thank you!"),
        "rUsT" => println!("So you appreciate Rust? That's great! Thank you!"),
        "No" => println!("So you like nothing? Alright then... :)"),
        "no" => println!("So you like nothing? Alright then... :)"),
        "NO" => println!("So you like nothing? Alright then... :)"),
        _ => println!("It seems that you like {syöte}."),
    }
}
