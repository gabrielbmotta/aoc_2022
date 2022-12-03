#include <iostream>
#include <fstream>

int score_from_move_1(char move)
{
    switch(move){
        case 'X':
            return 1;
        case 'Y':
            return 2;
        case 'Z':
            return 3;
        default:
            return 0;
    }
}

int score_from_match_1(char opponent_move, char your_move)
{
    int diff = (your_move - 23) - opponent_move;

    if (diff == 0){
        return 3; // tie
    } else if(diff == -1 || diff == 2) {
        return 0;
    } else if(diff == 1 || diff == -2) {
        return 6;
    }
}

int total_strategy_score_1(const char* file){
    std::ifstream input_file(file);

    if(input_file.is_open()){
        char opponent_move, your_move;
        int total_score = 0;
        while(input_file >> opponent_move >> your_move){
            total_score += score_from_move_1(your_move);
            total_score += score_from_match_1(opponent_move, your_move);
        }
        return total_score;
    }

    return -1;
}

int score_from_move_2(char move)
{
    switch(move){
        case 'A':
            return 1;
        case 'B':
            return 2;
        case 'C':
            return 3;
        default:
            return 0;
    }
}

int score_from_match_2(char opponent_move, char desired_result)
{
    switch(desired_result){
        case 'X':{
            if (opponent_move == 'A') return 3;
            if (opponent_move == 'B') return 1;
            if (opponent_move == 'C') return 2;
        }
        case 'Y':{
            return 3 + score_from_move_2(opponent_move);
        }
        case 'Z':{
            if (opponent_move == 'A') return 6 + 2;
            if (opponent_move == 'B') return 6 + 3;
            if (opponent_move == 'C') return 6 + 1;
        }
    }
}

int total_strategy_score_2(const char* file){
    std::ifstream input_file(file);

    if(input_file.is_open()){
        char opponent_move, your_move;
        int total_score = 0;
        while(input_file >> opponent_move >> your_move){
            total_score += score_from_match_2(opponent_move, your_move);
        }
        return total_score;
    }

    return -1;
}

int main(int argc, char* argv[])
{
    std::cout <<"Total Score from first strategy: " << total_strategy_score_1("input.txt") << "\n";
    std::cout <<"Total Score from second strategy: " << total_strategy_score_2("input.txt") << "\n";

    return 0;
}
