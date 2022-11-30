#include <stdio.h>

int main(int argc, char* argv[])
{
    printf("Hello world!\n");

    FILE *fp;
    fp = fopen("test.txt", "r");

    if(fp != NULL){
        char buff[2];
        while(!feof(fp) && fscanf(fp, "%c %c\n", &buff[0], &buff[1])){
            printf("%c, %c\n", buff[0], buff[1]);
        }
        return 0; 
    }

    return 0;
}