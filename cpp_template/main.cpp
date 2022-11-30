#include <iostream>
#include <fstream>

int main(int argc, char* argv[])
{
    std::cout << "Hello world!\n";

    std::ifstream input_file("test.txt");

    if(input_file.is_open()){
        char a,b;
        while(input_file >> a >> b){
            std::cout << a << ", " << b << "\n";
        }
        return 0;
    }

    return 1;
}
