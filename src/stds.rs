pub fn run() {
    let v = [1, 2, 3];
    let iter:Vec<i32> = v.into_iter().map(|x| {x+1}).collect();
    println!("{:?}", iter)
    
    
}