fn main() {

//-----------------------------------------------------------------------------

    // NOTE: How to creating variables
    
    // a is a 16 bit integer
    let a: i16 = 5;
       
//-----------------------------------------------------------------------------

    // NOTE: Mutability

    // In Rust variables are immutable by default.
    // This means that that they cannot be changed,
    // unless you explicty give them permission.
    // This is done using the keyword "mut"
    let mut b: i32 = 5;
    b = 10;

//-----------------------------------------------------------------------------

    // NOTE: Scope

    // scope
    let d: i32 = 30;
    
    {
        let e: i32 = d + 70; 
        println!("e is: {e}"); 
    }

    println!("d is: {d}");


//-----------------------------------------------------------------------------

    // NOTE: Shadowing - Example 1 (Shadowing in the same scope)
    
    let c: i32 = 10;
    
    // Because this variable has the same name as the c variable above,
    // and it is in the same scope, it will shadow c.
    let c: i32 = 20; 
    println!("c is: {c}");

    // In this example "c is 20" is what will be printed.
    // This is because the second c is shadowing the original c.
    // The second c will take the place of c until c goes out of scope
    
    // NOTE: When teaching shadowing I think teaching scope first
    // would make more sense.

//-----------------------------------------------------------------------------

    // NOTE: Shadowing Example 2 - Shadowing with different scopes

    // Shadowing is when you create a new variable using the `let` keyword,
    // that has the same name as an existing variable.
    
    let cars: i32 = 2;
    let books: u32 = 8;
    println!("The number of cars is {cars}"); // cars = 2 
    println!("The number of books is {books}"); // books = 8 
            
    {
        let cars = cars + 3;
        println!("The number of cars is {cars}"); 
        // cars = 5 within this scope
        
        // NOTE: When shadowing, because you are creating a new variable,
        // (only the name of the previous variable is copied),
        // you are free to use a different variable type if you want.
        // E.g. The original was a u32 but in this scope it is a &str type

        let books: &str = "red and green books"; 
        println!("I have the {books}"); 
        // I have the red and green books
    }
    
    println!("The number of cars is {cars}"); // cars is back to 2 again.
    println!("The number of books is {books}"); // books = 8  

//-----------------------------------------------------------------------------

}
