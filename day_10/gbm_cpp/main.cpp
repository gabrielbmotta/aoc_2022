#include <algorithm>
#include <cmath>
#include <fstream>
#include <iostream>
#include <set>
#include <string>
#include <utility>
#include <vector>

void validate_score(int &score, const std::vector<int> &roi, int r_n,
                    int count) {
  if (std::count(roi.begin(), roi.end(), r_n)) {
    std::cout << "r " << r_n << "rc " << count << " ";
    auto new_score = (count * r_n);
    std::cout << " - " << new_score << "\n";
    score += new_score;
  }
}

int main(int argc, char *argv[]) {
  std::ifstream infile("input.txt");
  std::string s;
  int value_to_add;
  int rolling_count = 1;
  int round_n = 0;
  int score = 0;

  std::vector<int> rounds_of_interest{20, 60, 100, 140, 180, 220};

  while (infile >> s) {
    if (s == "addx") {
      ++round_n;
      infile >> value_to_add;
      validate_score(score, rounds_of_interest, round_n, rolling_count);
      ++round_n;
      validate_score(score, rounds_of_interest, round_n, rolling_count);
      rolling_count += value_to_add;
    } else if (s == "noop") {
      ++round_n;
      validate_score(score, rounds_of_interest, round_n, rolling_count);
    }
  }

  std::cout << "Final score: " << score << "\n";

  return 0;
}
