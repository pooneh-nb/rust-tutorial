
// pub fn run() {
//     let number_list = vec![10,24,102,69,78];
//     let largest_num = find_max(&number_list);
    
//     println!("Largest number is {}", largest_num);
// }

// fn find_max<T: std::cmp::PartialOrd>(number_list:&[T]) -> T{
//     let mut largest_num = number_list[0];
//     for &num in number_list{
//         if num > largest_num{
//             largest_num = num;
//         }
//     }
//     largest_num
// }

// define struct to use generic
// #[derive(Debug)]
// pub struct Point<T, U> {
//     x: T,
//     y: U,
// }
// pub fn generic_filed_in_struct<T, U>(x_:T, y_:U) -> Point<T, U> {
//     let integer = Point {x: x_, y: y_};
//     integer
// }

// generics in enums
// enum Option<T, E> {
//     Ok(T),
//     Err(E),
//     None,
// }

// generics in method definitions
pub struct Point<T>{
    x: T,
    y: T,
}


impl<T> Point<T>{
    fn x(&self) -> &T {
        &self.x
    }
}

pub fn run(){
    let p = Point {x: 5, y: 10};
    println!("p.x = {}", p.x());
}