#include <stdio.h>

int main(void) {
    
    // Printing a simple message
    // The f in printf stands for formatted
    // \n means the next print statement after this
    // will be printed on a new line
    printf("Hello World\n");
    
    // Printing an integer
    int a = 42; 
   
    // %d means that a should be formatted as a decimal (base 10) number
    printf("The value of a is: %d\n", a);

    // How to format how an output is lined up
    // \t is what you use
    // \t means tab

    printf("The value of a is: \t%d\n", a);
    
    // NOTE: This is the output:
    // Hello World
    // The value of a is: 42
    // The value of a is:      42

}
