
// loops - used tto iterate until a condition is met

pub fn run(){
    let mut count = 1;

    // infinite loop
    // loop{
    //     count+=1;
    //     println!("Number: {}", count);
    //     if count == 20{
    //         break;
    //     }
    // }

    // while loop (FizzBuzz)
    // while count <= 100 {
    //     if count % 15 == 0{
    //         println!("fizzbuzz");
    //     } else if count %3 == 0{
    //         println!("fizz");
    //     } else if count % 5== 0{
    //         println!("buzz");
    //     } else {
    //         println!("{}", count);
    //     }

    //     //Inc
    //     count += 1;
    // }

    // for range
    // for x in 0..100{
    //     if x % 15 == 0{
    //         println!("fizzbuzz");
    //     } else if x %3 == 0{
    //         println!("fizz");
    //     } else if x % 5== 0{
    //         println!("buzz");
    //     } else {
    //         println!("{}", x);
    //     }
    // }
    
    // returning values from loops
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count *2;
        }
    };

    println!("The resut is {result}");



}