#include <iostream>
#include <fstream>
#include <string>

struct WorkAssignment{
    int min1;
    int max1;

    int min2;
    int max2;
};

bool readWorkAssignment(std::ifstream& input, WorkAssignment& wa)
{
    std::string temp;

    std::getline(input, temp, '-');
    if(input.eof()){
        return false;
    }
    wa.min1 = std::stoi(temp);


    std::getline(input, temp, ',');
    wa.max1 = std::stoi(temp);

    std::getline(input, temp, '-');
    wa.min2 = std::stoi(temp);

    std::getline(input, temp);
    wa.max2 = std::stoi(temp);

    // std::cout << wa.min1 << " " << wa.max1 << " " << wa.min2 << " " << wa.max2 << "\n";

    return true;
}

int num_of_contained_ranges(const char* in){
    std::ifstream input_file(in);
    if(!input_file.is_open()){
        return -1;
    }

    int sum = 0;
    WorkAssignment entry;
    while(readWorkAssignment(input_file, entry)){
        if((entry.min1 <= entry.min2 && entry.max1 >= entry.max2) ||
        (entry.min2 <= entry.min1 && entry.max2 >= entry.max1)){
            ++sum; 
        }
    }
    return sum;
}

int num_of_overlapping_ranges(const char* in){
    std::ifstream input_file(in);
    if(!input_file.is_open()){
        return -1;
    }

    int sum = 0;
    WorkAssignment entry;
    while(readWorkAssignment(input_file, entry)){
        if((entry.min1 <= entry.min2 && entry.min2 <= entry.max1) || (entry.min1 <= entry.max2 && entry.max2 <= entry.max1) ||
        (entry.min2 <= entry.min1 && entry.min1 <= entry.max2) || (entry.min2 <= entry.max1 && entry.max1 <= entry.max2)){
            ++sum; 
        }
    }
    return sum;
}

int main(int argc, char* argv[])
{
    std::cout << "Ranges that contain each other: " << num_of_contained_ranges("input.txt") << "\n";
    std::cout << "Ranges that overlap: " << num_of_overlapping_ranges("input.txt") << "\n";

    return 0;
}
