#include <iostream>
#include <fstream>
#include <string>

void max_calories(const char* input){
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
        std::cout << "Max calories = " << max_sum << "\n";
    }
}

void check_and_replace(int new_value, int* old_value, int num_values)
{
    for(int i = 0; i < num_values; ++i){
        if(new_value > old_value[i]){
            int temp = old_value[i];
            old_value[i] = new_value;
            check_and_replace(temp, &old_value[i], num_values - i);
            return;
        }
    }
}

void top_n_calories(const char* input, int top_n = 3){
    std::ifstream input_file(input);

    if(input_file.is_open()){
        int* top_n_sums = new int[top_n];
        int rolling_sum = 0;

        std::string temp;

        while(std::getline(input_file, temp)){
            if(temp.empty() && rolling_sum > 0){
                check_and_replace(rolling_sum, top_n_sums, top_n);
                rolling_sum = 0;
                continue;
            }
            rolling_sum += std::stoi(temp);
            temp.clear();
        }
        int combined_sum = 0;
        for(int i = 0; i < top_n; ++i){
            std::cout << i+1 << "-\t" << top_n_sums[i] << "\n";
            combined_sum += top_n_sums[i];
        }
        std::cout << "For a total of " << combined_sum << "\n";
    }
}

int main(int argc, char* argv[])
{
    max_calories("input.txt");
    top_n_calories("input.txt", 3);

    return 0;
}
