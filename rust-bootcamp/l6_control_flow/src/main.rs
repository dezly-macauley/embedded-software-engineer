fn main() {

    // if / else
    let a: i32 = 5;

    if a > 5 {
        println!("bigger than 5");
    } else if a > 3 {
        println!("bigger than 3");
    } else {
        println!("smaller or equal to 3");
    }

    // NOTE: How to to use and if statement to assign a value to a variable
   
    // If "a" is greater than 5, then b = 1
    // If "a" is not greater than 5, then b = -1;
    let b: i32 = if a > 5 { 1 } else { -1 };

//-----------------------------------------------------------------------------

    // NOTE: loop

    // This will execute forever until it reaches a break statement 

    loop {

        println!("loop forever");
        break;

    }

//-----------------------------------------------------------------------------

    // NOTE: loop (breaking out of a specific point)
    // This is done by labelling each loop

    'outer: loop {
        println!("loop forever");

        loop {
            break 'outer;
        }

    }
    

//-----------------------------------------------------------------------------

    // NOTE: How to return values from a loop
    // This is an infinite loop construct (loop) that immediately breaks with the value 5
   
    // The use case for this would be when you want try to retrieve some
    // kind of information mutiple times, for a specific number of times,
    // and then when it finally fails. 
    // You assign x the value.

    let x: i32 = loop {
        break 5;
    };


//-----------------------------------------------------------------------------

    // NOTE: While loop

    // This variable must be made mutable so that its value can be 
    // changed in the while loop
    let mut number_of_chocolates: i32 = 0;

    while number_of_chocolates < 5  {
        println!("Number of chocolates is {number_of_chocolates}");
        number_of_chocolates += 1;
    } 

    println!("Final number of chocolates is: {number_of_chocolates}");

//-----------------------------------------------------------------------------

    // NOTE: For loop

    let list_of_scores: [i32; 6] = [38, 32, 11, 82, 43, 90];

    for score in list_of_scores {
        println!("{}", score);
    }

//-----------------------------------------------------------------------------

}
