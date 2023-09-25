fn main() {
    println!("Hello World, welcome to Rust Programming");
    println!("I'm a Rustacean!");
    let x= 5 + 90 + 5;
    println!("Is `x` 10 or 100? x = {}", x);
    println!("{} days", 44);
    println!("{0}, this is {1}. {1}, I'm happy you met {0}.", "Faith", "Mubarak");
    println!("My name is {name}, I am {age:b} years old and I am a {title}.",
            name = "Shemang David Joshua",
            age = 30,
            title="Software Engineer");
    println!("{:0>6}", 33);
    println!("My name is {0}, {1} {0}", "Bond", "James");
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Structure(i32);
     println!("This struct `{:?}` won't print...", Structure(3));
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
    println!("Pi is roughly {pi:.3}", pi= 3.141592);
}