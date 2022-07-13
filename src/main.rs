mod print;
mod var;
mod types;
mod strings;
mod tuples;
mod arrays;
mod vectors;
mod conditionals;
mod loops;
mod functions;
mod pointers;
mod structs;
mod enums;
mod cli;
mod closure;
mod iteration;

fn main(){
    // print::run();
    // var::run();
    //types::run();
    // strings::run();
    // tuples::run();
    // arrays::run();
    // vectors::run();
    // conditionals::run();
    // loops::run();
    // functions::run();
    pointers::run();
    let name = String::from("ownership");
    pointers::function_ownership(name.clone());
    println!("{}", name)
    // structs::run();
    // enums::run();
    // cli::run();
    // closure::run();
    // iteration::run();
}
