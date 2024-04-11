    // NOTE:  Constants

    // In Rust constants are always immutable. 
    // You cannot use the keyword "mut" with a constant
    
    const MAX_PLAYERS: u8 = 10;

//-----------------------------------------------------------------------------

    // NOTE: Static Variables
    // Unlike Constants, static variables can be marked as mutable 

    static mut COMPANY_NAME: &str = "Thunder Thrust Inc";

    // This is unsafe. If you use a mutable static variable,
    // make sure to add it to an unsafe block

//-----------------------------------------------------------------------------

fn main() {
   
        // NOTE: How to use a constant
        // Constants do not occupy a specific location in memory
        let a: u8 = MAX_PLAYERS;

        // NOTE: How to use statics
        // Statics occupy a specific location in memory
        // This means that there is only one instance of the value
        
        // Static variables must be put in an unsafe block

        unsafe {
            let c: &str = COMPANY_NAME;
        }



}
