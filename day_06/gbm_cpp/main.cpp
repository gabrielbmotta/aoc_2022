#include <iostream>
#include <fstream>
#include <deque>

int find_start_of_packet(const char* infile, int packet_size)
{
    std::ifstream input_file(infile);

    if(input_file.is_open()){
        std::deque<char> past_inputs;
        char input;
        int index = 0;
        int unique['z' - 'a' + 1] = {0};

        while(input_file >> input){
            ++index;
            past_inputs.push_back(input);
            if(past_inputs.size() < packet_size){
                continue;
            }

            bool found_duplicate = false;
            std::fill_n(unique, 'z' - 'a' + 1, 0);
            for(int i = 0; i < packet_size; ++i){
                ++unique[past_inputs[i] - 'a'];
                if(unique[past_inputs[i] - 'a'] > 1){
                    found_duplicate = true;
                    break;
                }
            }
            if(!found_duplicate){
                return index;
            }

            past_inputs.pop_front();
        }
    }

    return -1;
}

int main(int argc, char* argv[])
{
    std::cout << "First 4 char group:\t" << find_start_of_packet("input.txt", 4) <<  "\n";
    std::cout << "First 14 char group:\t" << find_start_of_packet("input.txt", 14) <<  "\n";
}
