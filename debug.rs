#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

fn main() {
    let name = "Peter".to_string();
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);
}