fn main() {

    // NOTE: In Rust functions can be called before they have been declared,
    // as long as the compiler knows where to find the function
   
    my_function(22);
    
    let b: i32 = add_two(5,10); 
    println!("The value of b is: {b}");    

}


// This function does not return anything.
fn my_function (x: i32) {
    println!("my_function called with: {}", x);
}

// This is a function that returns an i32
fn add_two(_num1: i32, _num2: i32) -> i32 {
    let result = _num1 + _num2; 
    
    // In Rust the last line of a function is explicitly returned
    // Just don't remember NOT to add the semicolon
    result

}

