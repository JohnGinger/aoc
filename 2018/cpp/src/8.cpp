#include <iostream>
#include <fstream>
#include <set>
#include <vector>
#include "util.h"
#include <map>
#include <regex>
#include <unordered_map>
#include <ctime>
#include <numeric>
#include <sstream>
using namespace std;

namespace day8
{

vector<int> get_string_as_numbers(string input){
istringstream iss(input);
    vector<int> numbers;
    while (iss)
    {
        string word;
        iss >> word;
        try{
            numbers.push_back(stoi(word));
        }
            
        catch(const std::exception&) {
            cout << "Could not parse " << word << endl;
        }
    } ;
    return numbers;
}

pair<int, vector<int>> get_metadata(vector<int> input, int number_children){
    int num_children = input[0];
    int num_metadata = input[1];

    metadata = input[]
}


void run()
{
    auto lines = get_input_as_vec("8");
    auto input = lines[0];
    auto numbers = get_string_as_numbers(input);
    cout << "Part 1 is placeholder" << endl;
    cout << "Part 2 is placeholder" << endl;
}
} // namespace day7