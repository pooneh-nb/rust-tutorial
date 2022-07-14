pub fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

pub fn placeholder(dice_roll: u8) {
    match dice_roll {   
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
}

fn add_fancy_hat() {println!("add fancy hat");}
fn remove_fancy_hat() {println!("remove fancy hat");}
fn move_player(num_spaces: u8){println!("{} move player", num_spaces);}

pub fn match_if_let(config_max:Option<u8>){
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
}

pub fn just_if_let(config_max:Option<u8>){
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}

