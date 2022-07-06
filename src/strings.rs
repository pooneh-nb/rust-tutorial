// Primitive str: immutable fixed-length string somewhere in memoru
// String = Growable, heap allocated data structure - use when you need to modify or own string data
pub fn run(){
    let hello = "Hello"; // str
    let mut hello_2 = String::from("Hello "); // String

    println!("{}", hello);
    println!("{}", hello_2);

    // Get length
    println!("Length {}", hello.len());

    // Push char
    hello_2.push('W');
    println!("{}", hello_2);

    // push strting
    hello_2.push_str("orld");
    println!("{}", hello_2);

    // Capacity in bytes
    println!("Capacity: {}", hello_2.capacity());

    // check if empty
    println!("Is empty: {}", hello_2.is_empty());

    // contains     
    println!("Contains 'World'{}", hello_2.contains("World"));

    // Replcace
    println!("Replace: {}", hello_2.replace("World", "There"));

    // Loop through string by whitespace
    for word in hello_2.split_whitespace(){
        println!("{}", word);
    }

}