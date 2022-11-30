#include <stdio.h>

int main(int argc, char* argv[])
{
    printf("Hello world!\n");

    // We open a file test.txt
    FILE *fp;
    fp = fopen("test.txt", "r");

    // We check if fp has succesfully opened a file.
    if(fp != NULL){

        char input1, input2;// these variables will hold out file inputs

        // feof checks if we've reached the end of the file, fscanf reads in the expected pattern from the file
        while(!feof(fp) && fscanf(fp, "%c %c\n", &input1, &input2)){
            // and we printto screen
            printf("%c, %c\n", input1, input2);
        }

        // And in c we clean up after ourselves.
        fclose(fp);
        return 0; 
    }

    return 1;
}