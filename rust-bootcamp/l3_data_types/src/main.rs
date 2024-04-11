fn main() {
    
    // NOTE: Data Types

    //-------------------------------------------------------------------------

    // NOTE: boolean (true or false)
    let b1: bool = true;

    //-------------------------------------------------------------------------
    
    // NOTE: Unsigned integers (Only positive whole numbers)

    // Unsigned Integer (positive numbers only)
    // Variants include: u8, u16, u32, u64, u128
    
    // a is Unsigned integer, 
    // more specifically it is an 8 bit unsigined integer
    // and the value of a is 25

    let a: u8 = 25;
    
    //-------------------------------------------------------------------------

    // NOTE: Signed integer (negative and positive whole numbers) 

    // Variants: i8, i16, i32, i64, i128  

    let i7: i8 = 8; 

    //-------------------------------------------------------------------------
   
    // NOTE:  Floating point numbers (numbers that have a decimals)

    let f1: f32 = 1.0;
    let f2: f64 = 1.0;

    //-------------------------------------------------------------------------
    
    // NOTE: Platform specific integers 

    // A pointer-sized unsigined integer.
    // The size of this primitive data type is how many bytes it takes to 
    // reference any location in memory. 
    // For example, on a 32 bit target, this is 4 bytes.
    // And on a 64 bit this is 8 bytes.
    
    let p1: usize = 1;

    // A pointer-sized singed integer
    // The size of this primitive data type is how many bytes it takes to 
    // reference any location in memory. 
    // For example, on a 32 bit target, this is 4 bytes.
    // And on a 64 bit this is 8 bytes.
    let p2: isize = 2;

    //-------------------------------------------------------------------------
    
    // NOTE: Characters, &str, and String
   
    // This is a character
    let c1: char = 'c';

    // This is a string slice
    // All string literals are string slices
    let s1: &str = "hello";

    // This is a String type
    let s2: String = String::from("hello");

    //-------------------------------------------------------------------------
   
    // NOTE: Arrays 
    // Array hold multiple values of the same type
   
    // a1 is an array that holds 5 values
    // Each of those values is a 32 bit signed integer
    let a1: [i32; 5] = [51, 12, 33, 42, 15];
    
    let first_number = a1[0]; // 51
    let last_number = a1[4]; // 15 

    //-------------------------------------------------------------------------
    
    // NOTE: Tuples
    // Tuples hold multiple values of different types

    let my_tuple: (i32, f64, &str) = (15, 2.9, "Dezly");

    let first_value: i32 = my_tuple.0;
    println!("The first value of my_tuple is: {first_value}");

    //-------------------------------------------------------------------------

    // NOTE: Destructuring Tuples 

    // Destructuring a tuple simply means that you take each value in a tuple
    // and and assign each one to a seperate variable.

    // E.g. Let's say you had a tuple that contained 3 values.
    // You could create 3 variables that will store each of those values
    
    let player_stats: (i32, &str, f64) = (12, "Jake", 36.56);

    let (player_index, player_name, player_rating) = player_stats;
    
    println!("Index of player is: {player_index}");
    println!("Name of player is: {player_name}");
    println!("Rating of player is: {player_rating}");

    //-------------------------------------------------------------------------
    
    // NOTE: An empty tuple is called a unit type

    let unit: () = ();
    
    // Unit types are usually returned implicitly when no other meaningful
    // value could be returned.
    // E.g. functions that don't return a value,
    // implicitly return the unit type.

    //-------------------------------------------------------------------------
    
    // NOTE: Type aliases 
    // This is when you create a more convienient name for a variable type
    // that already exists.
   
    // The naming convention is UpperCamelCase 
    type AgeType = u8;

    let user_age: AgeType = 57;



    //-------------------------------------------------------------------------

}
