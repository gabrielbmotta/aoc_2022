#include <iostream>
#include <fstream>
#include <vector>

int getPriority(char letter)
{
    if (letter < 'a') {
        return letter - 38;
    } else {
        return letter - 96;
    }
}

int maxPriorityBetweenCompartments(const std::string& rucksack)
{
    const int len = rucksack.length();
    for(int i = 0; i <= len/2; ++i){
        for(int j = len/2; j < len; ++j){
            if(rucksack[i] == rucksack[j]){
                return getPriority(rucksack[i]);
            }
        }
    }
    return 0;
}

int sumOfRepeatedItemPriority(const char* infile){
    std::ifstream input_file(infile);
    if(!input_file.is_open()){
        return -1;
    }

    std::string temp;
    int rolling_sum = 0;
    while(std::getline(input_file, temp)){
        rolling_sum += maxPriorityBetweenCompartments(temp);
    }
    return rolling_sum;
}

int sumOfGroupPriority(const char* infile, int group_size){
    std::ifstream input_file(infile);
    if(!input_file.is_open()){
        return -1;
    }

    std::vector<std::string> group;
    group.resize(group_size);

    int rolling_sum = 0;
    int group_fill_index = 0;
    while(std::getline(input_file, group[group_fill_index])){
        if(group_fill_index < group_size - 1){
            ++group_fill_index;
            continue;
        }

        std::vector<std::vector<bool>> priority_tracker;
        priority_tracker.resize(53);
        for(auto &tracker : priority_tracker){
            tracker.resize(group_size);
        }

        for (int i = 0; i < group.size(); ++i){
            for(int j = 0; j < group[i].size(); j++){
                priority_tracker[getPriority(group[i][j])][i] = true;
            }
        }

        for(int i = 1; i < 53; ++i){
            bool check = true;
            for(auto element : priority_tracker[i]){
                check *= element;
            }
            if(check){
                rolling_sum += i;
                break;
            }
        }
        group_fill_index = 0;
    }
    
    return rolling_sum;
}

int main(int argc, char* argv[])
{
    int part_1_sum = sumOfRepeatedItemPriority("input.txt");
    std::cout << "Part 1 total: " << part_1_sum << "\n";

    int part_2_sum = sumOfGroupPriority("input.txt", 3);
    std::cout << "Part 2 total: " << part_2_sum << "\n";
    
    return 0;
}
