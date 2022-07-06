// Conditionals - used to check the condition of something and act  

pub fn run(){
    let age = 18;
    let check_id = true;
    let knows_person_of_age = true;

    // if-else
    if age >= 21 && check_id || knows_person_of_age{
        println!("Bartender: what would you like to drink?");
    } else if age < 21 && check_id{
        println!("Bartender: Sorry. you have to leave");
    } else {
        println!("Bartender: I need to see your ID");
    }

    //shorhand if
    let is_of_age = if age >=21 {true} else {false};
    println!("Is of age: {}", is_of_age);
}