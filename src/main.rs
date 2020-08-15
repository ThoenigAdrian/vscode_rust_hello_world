fn main() {
    println!("Hello World!");
    // Wait for user to hit enter so the window doesn't dissapear immediately.
    std::io::stdin().read_line(&mut String::new()).expect("Failed to read line");
}