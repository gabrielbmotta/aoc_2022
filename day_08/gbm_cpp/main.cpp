#include <iostream>
#include <fstream>
#include <string>
#include <vector>


std::vector<std::vector<int>> get_map(const char* infile)
{
    std::ifstream input_file(infile);
    std::string temp;
    std::vector<std::vector<int>> my_vec;

    int row_index = 0;
    while(std::getline(input_file, temp)){
        my_vec.push_back({});
        for(auto character : temp){
            my_vec[row_index].push_back(character - '0');
            //std::cout << my_vec[row_index].back();
        }
        //std::cout << "\n";
        ++row_index;
    }
    
    return my_vec;
}

bool can_be_seen(const std::vector<std::vector<int>>& tree_map, int row, int column)
{
    if(row == 0 || row == tree_map.size() - 1){
        return true;
    }
    if(column == 0 || column == tree_map[row].size() - 1){
        return true;
    }

    bool blocked1 = false;
    for(int i = 0 ; i < row; ++i){
        if(tree_map[i][column] >= tree_map[row][column]){
            blocked1 = true;
            break;
        }
    }

    bool blocked2 = false;
    for(int i = row + 1 ; i < tree_map.size(); ++i){
        if(tree_map[i][column] >= tree_map[row][column]){
            blocked2 = true;
            break;
        }
    }

    bool blocked3 = false;
    for(int i = 0 ; i < column; ++i){
        if(tree_map[row][i] >= tree_map[row][column]){
            blocked3 = true;
            break;
        }
    }

    bool blocked4 = false;
    for(int i = column + 1; i < tree_map[row].size(); ++i){
        if(tree_map[row][i] >= tree_map[row][column]){
            blocked4 = true;
            break;
        }
    }

    return !blocked1 + !blocked2 + !blocked3 + !blocked4;
}

int scenic_tree_score(const std::vector<std::vector<int>>& tree_map, int row, int column)
{
    int score1 = 0;
    for(int i = row - 1 ; i >= 0 ; --i){
        if(tree_map[i][column] < tree_map[row][column]){
            ++score1;
        } else if(tree_map[i][column] == tree_map[row][column]){
            ++score1;
            break;
        } else {
            break;
        }
    }

    int score2 = 0;
    for(int i = row + 1 ; i < tree_map.size() ; ++i){
        if(tree_map[i][column] < tree_map[row][column]){
            ++score2;
        } else if(tree_map[i][column] == tree_map[row][column]){
            ++score2;
            break;
        } else {
            break;
        }
    }

    int score3 = 0;
    for(int i = column - 1 ; i >= 0 ; --i){
        if(tree_map[row][i] < tree_map[row][column]){
            ++score3;
        } else if(tree_map[row][i] == tree_map[row][column]){
            ++score3;
            break;
        } else {
            break;
        }
    }

    int score4 = 0;
    for(int i = column + 1 ; i < tree_map[row].size() ; ++i){
        if(tree_map[row][i] < tree_map[row][column]){
            ++score4;
        } else if(tree_map[row][i] == tree_map[row][column]){
            ++score4;
            break;
        } else {
            break;
        }
    }

    return score1 * score2 * score3 * score4;
}

int main(int argc, char* argv[])
{
    // We create an input stream from the file test.txt
    auto my_map = get_map("input.txt");

    int sum_of_seen = 0;
    for (int i = 0; i < my_map.size(); ++i){
        for(int j = 0; j < my_map[i].size(); ++j){
            if(can_be_seen(my_map, i, j)){
                ++sum_of_seen;
            }
        }
    }

    std::cout << "Number of seen trees: " << sum_of_seen << "\n";

    int highest_score = 0;
    for (int i = 0; i < my_map.size(); ++i){
        for(int j = 0; j < my_map[i].size(); ++j){
            int score = scenic_tree_score(my_map, i, j);
            std::cout << score << "\n";
            if(score > highest_score){
                highest_score = score;
            }
        }
    }

    std::cout << "Highest scenic score: " << highest_score << "\n";

    return 0;
}
