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
mod matches;
mod generics;

use generics::Point;


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

    // pointers::run();
    // let name = String::from("ownership");
    // pointers::function_ownership(name.clone());
    // // println!("{}", name);
    // let s1 = pointers::gives_ownership();
    // let s2 = String::from("hello");
    // let s3 = pointers::takes_and_gives_ownership(s2.clone());
    // println!("{}", s2);

    // structs::run();
    // enums::run();
    // cli::run();
    // closure::run();
    // iteration::run();

    // let five = Some(5);
    // let six = matches::plus_one(five);
    // let none = matches::plus_one(None);
    // println!("{:?} {:?} {:?}", five, six, none);

    // let dice_roll = 9;
    // matches::placeholder(dice_roll);

    // let config_max  = Some(3u8);
    // //matches::match_if_let(config_max);
    // matches::just_if_let(config_max);
    // generics::run();
    // let integer = generics::generic_filed_in_struct(5,6);
    // println!("{:?}", integer);
    // let float = generics::generic_filed_in_struct(5.5, 6.5);
    // println!("{:?}", float);
    // let mix = generics::generic_filed_in_struct(5, "Pouneh");
    // println!("{:?}", mix);

    generics::run();
}
