#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <utility>

struct node{
  bool visited = false;
};

struct instr{
  char dir;
  int num;
};

int main(int argc, char* argv[])
{
  std::ifstream input_file("input.txt");

  int num;
  char dir;
  
  std::vector<instr> directions;
  int total = 0;

  while(input_file >> dir >> num){
    // std::cout << dir << " " << num << "\n";
    directions.push_back({dir,num});
    total += num;
  }

  std::cout << total << "\n";

  std::vector<std::vector<node>> map;
  int map_size = total * 2 + 1;
  map.resize(map_size);
  for(auto& element : map){
    element.resize(map_size);
  }

  int curr_x = total, curr_y = total;

  for(auto& instruction : directions){
    // do things here
  }
  
  return 0;
}
