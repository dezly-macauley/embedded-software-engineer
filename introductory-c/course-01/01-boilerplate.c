#include <stdio.h>

// "void" means that the function takes no arguements
// This means that it does not require any inputs to work
int main(void) {

    printf("Hello World\n");

    return 0;
}

// NOTE: What is "boilerplate"?

// Boilerplate is the minimum required code structure
// to create a basic program that works in a specific programming language.

// An example of a basic program would be a program that displays the message
// "Hello World" when it is run

// NOTE: How to compile a C program?

// First make sure that you are in the directory that contains the .c file
// where your code is written.

// Then open the terminal and run the following:
// gcc name_of_your_program.c -o what_you_want_the_output_file_to_be_called

// E.g. gcc my-awesome-code.c -o my-awesome-code

// This will create a binary file (On Linux this file has no extension)

// NOTE: How to run a C program after it has been compiled to binary?

// To run this binary file: ./name_of_file

// eg. ./awesome_code
