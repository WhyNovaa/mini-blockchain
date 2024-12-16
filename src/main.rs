mod models;
mod tools;

fn main() {
    println!("Hello, world!");
    let date = chrono::offset::Local::now().date_naive().to_string();
}
