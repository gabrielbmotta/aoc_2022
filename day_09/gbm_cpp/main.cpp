#include <cmath>
#include <fstream>
#include <iostream>
#include <set>
#include <string>
#include <utility>
#include <vector>

typedef std::pair<int, int> pos;

int distance(pos p1, pos p2) {
  int dist =
      sqrt(pow((p1.first - p2.first), 2) + pow((p1.second - p2.second), 2));
  // std::cout << "d: " << dist << "\n";
  return dist;
}

void move_head(pos &head, char direction) {
  switch (direction) {
  case 'R':
    ++head.first;
    break;
  case 'L':
    --head.first;
    break;
  case 'U':
    ++head.second;
    break;
  case 'D':
    --head.second;
    break;
  }
}

void move_tail(const pos &head, pos &tail, char direction) {
  tail = head;
  switch (direction) {
  case 'R':
    --tail.first;
    break;
  case 'L':
    ++tail.first;
    break;
  case 'U':
    --tail.second;
    break;
  case 'D':
    ++tail.second;
    break;
  }
}

void traverse(const char *input_file_path) {
  std::ifstream input_file(input_file_path);

  pos head_pos{0, 0}, tail_pos{0, 0};

  std::set<pos> tail_pos_tracker;

  int num;
  char dir;
  while (input_file >> dir >> num) {
    for (int i = 0; i < num; ++i) {
      std::cout << "[" << head_pos.first << "," << head_pos.second << " -- "
                << tail_pos.first << "," << tail_pos.second << "]\n";
      if (tail_pos_tracker.insert(tail_pos).second) {
        //  std::cout << tail_pos.first << "," << tail_pos.second << "\n";
      }

      move_head(head_pos, dir);

      // std::cout << "POST-MOVE[" << head_pos.first << "," << head_pos.second
      // << " " << tail_pos.first << "," << tail_pos.second << "]\n";
      if (distance(head_pos, tail_pos) > 1) {
        // std::cout << "MOVING\n";
        move_tail(head_pos, tail_pos, dir);
      }
    }
    tail_pos_tracker.insert(tail_pos);
  }
  tail_pos_tracker.insert(tail_pos);

  std::cout << "Num places visited: " << tail_pos_tracker.size() << "\n";
}

int main(int argc, char *argv[]) {
  traverse("input.txt");

  return 0;
}
