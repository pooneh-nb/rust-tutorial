// Reference Pointers: Point to a resource in memory

pub fn run() {
    // Primitive Array
    let arr1 = [1,2,3];
    let arr2 = arr1;

// with non-primitives, if you assign another variable to a piece of data, the first variable will no longer hold that value.
// you'll need to use a reference (&) to point to the resource

// vector
    let vec1 = vec![1,2,3];
    // if we say : let vec2 = vec1; hen the owner goes out of scope, the value will be dropped. it means that the vec1 is not the owner anymore
    let vec2 = &vec1;
    let vec3 = vec1.clone();


    println!("values: {:?}", (&vec1, vec2, vec3));

}

pub fn function_ownership(name:String){
    let str = name;
    println!("{}", str);
}

pub fn gives_ownership() -> String{
    let some_string = String::from("yours");
    some_string
}

pub fn takes_and_gives_ownership(a_string: String) -> String{
    a_string
}