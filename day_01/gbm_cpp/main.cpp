#include <iostream>
#include <fstream>
#include <string>

int max_calories(const char* input){
    std::ifstream input_file(input);

    if(input_file.is_open()){
        int max_sum = 0;
        int rolling_sum = 0;

        std::string temp;

        while(std::getline(input_file, temp)){
            if(temp.empty() && rolling_sum > 0){
                if(rolling_sum > max_sum){
                    max_sum = rolling_sum;
                }
                rolling_sum = 0;
                continue;
            }
            rolling_sum += std::stoi(temp);
            temp.clear();
        }
        return max_sum;
    }

    return -1;
}

int main(int argc, char* argv[])
{
    auto max_cal = max_calories("input.txt");
    std::cout << "Max calories = " << max_cal << "\n";

    return 0;
}
