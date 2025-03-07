fn main() {
    let s: &str = "Hello World!";

    print!("{}\n", s);
    println!("{}", s);

    eprint!("{}\n", s);
    eprintln!("{}\n", s);

    let name: &str = "Kim";
    println!("{name}");

    let formatted_string: String = format!("Hello {name}, nice to meet you!");
    println!("{formatted_string}");
}
