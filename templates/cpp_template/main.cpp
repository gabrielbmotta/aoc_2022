#include <iostream>
#include <fstream>

int main(int argc, char* argv[])
{
    std::cout << "Hello world!\n";

    // We create an input stream from the file test.txt
    std::ifstream input_file("test.txt");

    // We check if the file was succesfully opened.
    if(input_file.is_open()){

        char input1, input2; // these variables will hold our file inputs

        // Loop as long as we are able to read two characters in a line from the input
        while(input_file >> input1 >> input2){
            // and we print them to screen
            std::cout << input1 << ", " << input2 << "\n";
        }
        return 0;
    }

    return 1;
}
