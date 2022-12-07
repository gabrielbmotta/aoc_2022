#include <cctype>
#include <cstddef>
#include <cstdint>
#include <iostream>
#include <fstream>
#include <string>
#include <sys/types.h>
#include <unordered_set>

struct tree_node{
    std::unordered_set<tree_node*> sub_nodes;
    tree_node* parent;
    std::string name;
    uint size = 0;
};

tree_node* create_tree_from_file(std::ifstream& input_file){
    tree_node* current_node = nullptr;
    tree_node* root_node = nullptr;

    std::string temp;
    while(std::getline(input_file, temp)){
        if(!temp.empty() && temp[0] == '$')
        {
            if(temp[2] == 'c'){
                auto dirname = temp.substr(5);
                //std::cout << dirname;

                if(dirname == ".."){
                    current_node = current_node->parent;
                } else if (dirname == "/"){
                    if(root_node == nullptr){
                        root_node = new tree_node;
                        root_node->name = dirname;
                        root_node->parent = nullptr;
                    }
                    current_node = root_node;
                } else {
                    bool found_node = false;
                    for(auto* node : current_node->sub_nodes){
                        if(node->name == dirname){
                            current_node = node;
                            break;
                        }
                    }
                    // if(!found_node){
                    //     auto*new_node = new tree_node;
                    //     new_node->name = dirname;
                    //     new_node->parent = current_node;
                        
                    //     current_node->sub_nodes.insert(new_node);
                    //     current_node = new_node;
                    // }
                }
            } else if(temp[2] == 'l'){
                bool output_done = false;
                while(!output_done){
                    //std::cout << "parsing ouput\n";
                    auto next_char = input_file.peek();
                    //std::cout << (char)next_char << "\n";
                    if(next_char == '$'){
                        output_done = true;
                        continue;
                    } else {
                        std::string ls_output;
                        std::getline(input_file, ls_output);

                        if(std::isdigit(ls_output[0])){
                            bool unique = true;

                            auto sp_index = ls_output.find(' ');
                            auto new_name = ls_output.substr(sp_index + 1);

                            auto*new_node = new tree_node;
                            new_node->name = new_name;
                            new_node->size = std::stoi(ls_output);
                            new_node->parent = current_node;

                            current_node->sub_nodes.insert(new_node);
                        } else if(ls_output[0] == 'd'){
                            auto new_name = ls_output.substr(4);
                            
                            auto*new_node = new tree_node;
                            new_node->name = new_name;
                            new_node->parent = current_node;
                            //std::cout << new_name << "\n";

                            current_node->sub_nodes.insert(new_node);
                        } else {
                            output_done = true;
                        }
                    }
                }
                //std::cout << "done\n";
            }
        }
    }

    return root_node;
}

uint calculate_node_size(tree_node* node){
    int size = 0;
    if(node->size != 0){
        return node->size;
    }
    for (auto* sub_node : node->sub_nodes){
        size += calculate_node_size(sub_node);
    }
    node->size = size;
    return node->size;
}

void combinded_size_below(tree_node* root, uint threshold, int& sum)
{
    if(!root->sub_nodes.empty() && root->size < threshold){
        sum += root->size;
        //std::cout << root->name << " : " << root->size << "\n";
        //std::cout << "ON -- " << root->name << "\n";
    }

    for(auto* node : root->sub_nodes){
        //std::cout << node->name << "\n";
        combinded_size_below(node, threshold, sum);
    }
}

int size_of_dir(const char* infile, int threshold)
{
    std::ifstream input_file(infile);
    if(!input_file.is_open()){
        return -1;
    }

    auto* tree = create_tree_from_file(input_file);

    calculate_node_size(tree);

    int sum = 0;

    combinded_size_below(tree, threshold, sum);
    
    return sum;
}

void smalleset_dir_larger_than(tree_node* root, int threshold, uint& value)
{
    if(!root->sub_nodes.empty() && root->size >= threshold && root->size < value){
        value = root->size;
        // std::cout << "ON -- " << root->name << "\n";
    }

    for(auto* node : root->sub_nodes){
        // std::cout << node->name << "\n";
        smalleset_dir_larger_than(node, threshold, value);
    }
}

int smallest_dir_to_delete(const char* infile){
    std::ifstream input_file(infile);
    if(!input_file.is_open()){
        return -1;
    }

    auto* tree = create_tree_from_file(input_file);

    calculate_node_size(tree);

    uint value = UINT32_MAX;

    smalleset_dir_larger_than(tree, 30000000 - (70000000-tree->size), value);
    
    return value;
}

int main(int argc, char* argv[])
{
    std::cout << "Size of dirs below 100000:\t" << size_of_dir("input.txt", 100000) <<  "\n";

    std::cout << "Size of smallest dir to delete:\t" << smallest_dir_to_delete("input.txt") <<  "\n";
    
}
