#include <iostream>
#include <fstream>
#include <string>
#include <deque>
#include <vector>

std::vector<std::deque<char>>get_stacks(std::ifstream& infile)
{
    std::vector<std::deque<char>> stacks;

    bool done = false;
    std::string line;
    bool initialized = false;

    while(!done){
        std::getline(infile, line);
        int len = line.size();

        if(!initialized){
            stacks.resize(((len + 1) / 4) + 1);
            initialized = true;
        }

        if(line.empty()){
            done = true;
            break;
        }

        int index = 1;
        for(int i = 1; i < len;){
            if('A' <= line[i] && line[i] <= 'Z'){
                stacks[index].push_front(line[i]);
            }
            i += 4;
            ++index;
        }
    }

    return stacks;
}

// Leaving this here for shame :)
//
// std::vector<std::deque<char>>get_stacks()
// {
// /*
// Doing some cheating here
//     [W]         [J]     [J]        
//     [V]     [F] [F] [S] [S]        
//     [S] [M] [R] [W] [M] [C]        
//     [M] [G] [W] [S] [F] [G]     [C]
// [W] [P] [S] [M] [H] [N] [F]     [L]
// [R] [H] [T] [D] [L] [D] [D] [B] [W]
// [T] [C] [L] [H] [Q] [J] [B] [T] [N]
// [G] [G] [C] [J] [P] [P] [Z] [R] [H]
//  1   2   3   4   5   6   7   8   9 
// */

//     return {{},
//         {'G', 'T', 'R', 'W'},
//         {'G', 'C', 'H', 'P', 'M', 'S', 'V', 'W'},
//         {'C', 'L', 'T', 'S', 'G', 'M'}, 
//         {'J', 'H', 'D', 'M', 'W', 'R', 'F'},
//         {'P', 'Q', 'L', 'H', 'S', 'W', 'F', 'J'},
//         {'P', 'J', 'D', 'N', 'F', 'M', 'S'},
//         {'Z', 'B', 'D', 'F', 'G', 'C', 'S', 'J'},
//         {'R', 'T', 'B'},
//         {'H', 'N', 'W', 'L', 'C'}};
// }

int rearrange_items(const char* in){
    std::ifstream input_file(in);
    if(!input_file.is_open()){
        return -1;
    }

    std::string temp;
    int source, destination, amount;

    auto my_stacks = get_stacks(input_file);

    while(input_file >> temp >> amount >> temp >> source >> temp >> destination){
        for(int i = 0; i < amount; ++i){
            my_stacks[destination].push_back(my_stacks[source].back());
            my_stacks[source].pop_back();
        }
    }

    for(auto i = 1; i < my_stacks.size(); ++i){
        std::cout << my_stacks[i].back();
    }
    std::cout << "\n";
    
    return 0;
}

int rearrange_items_keeping_order(const char* in){
    std::ifstream input_file(in);
    if(!input_file.is_open()){
        return -1;
    }

    std::string temp;
    int source, destination, amount;

    auto my_stacks = get_stacks(input_file);

    while(input_file >> temp >> amount >> temp >> source >> temp >> destination){
        std::deque<char> temp_holder;
        for(int i = 0; i < amount; ++i){
            temp_holder.push_front(my_stacks[source].back());
            my_stacks[source].pop_back();
        }
        for(auto item : temp_holder){
            my_stacks[destination].push_back(item);
        }
    }

    for(auto i = 1; i < my_stacks.size(); ++i){
        std::cout << my_stacks[i].back();
    }
    std::cout << "\n";
    
    return 0;
}

int main(int argc, char* argv[])
{
    std::cout << "Part 1: ";
    rearrange_items("input.txt");
    std::cout << "\n";

    std::cout << "Part 2: ";
    rearrange_items_keeping_order("input.txt");
    std::cout << "\n";

    return 0;
}
