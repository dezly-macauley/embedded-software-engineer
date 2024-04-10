#include <stdio.h>

int main(void) {

    int numberOfHours = 40;
    int numberOfDays = 7;

    printf("You worked %d hours in %d days\n", numberOfHours, numberOfDays);

    // (float) numberOfHours is how to convert an integer to a float

    float averageHours = (float)numberOfHours / (float)numberOfDays;

    

    
    return 0;
}
