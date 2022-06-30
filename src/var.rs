// variables hold primitive data or references to data
// variables are immutable by default
// rust is a block-scoped language

pub fn run(){
    let name = "Brad";
    
    let mut age= 37;
    age = 38; 
    // => error: cannot assign twice to immutable variable
    // you need to add mut to the variable to be able to change it
    println!("My name is {} and I'm {}", name, age);


    //Define constant
    const ID:i32 = 0001;
    println!("ID: {}", ID);

    // Assign multiple variables 
    let (my_name, my_age) = ("Brad", 37);
    println!("{} is {}", my_name, my_age);
}