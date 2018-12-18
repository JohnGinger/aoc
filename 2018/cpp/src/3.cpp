#include <iostream>
#include <fstream>
#include <unordered_set>
#include <vector>
#include "util.h"
#include <map>
#include <regex>
#include <unordered_map>

using namespace std;

namespace day3
{

void run()
{
    std::regex number_regex("([0-9]+)");

    auto lines = get_input_as_vec("3");

    // vector<string> lines = {"#1 @ 1,3: 4x4",
    //                         "#2 @ 3,1: 4x4",
    //                         "#3 @ 5,5: 2x2"};
    auto fabric = unordered_map<int, int>();
    auto overlapping = unordered_set<int>();
    auto claims = unordered_set<int>();

    auto overlapping_count = 0;
    vector<int> numbers = {};
    for (auto line : lines)
    {
        auto numbers_begin = sregex_iterator(line.begin(), line.end(), number_regex);
        auto numbers_end = sregex_iterator();

        for (auto i = numbers_begin; i != numbers_end; ++i)
        {
            auto match = *i;
            numbers.push_back(stoi(match.str()));
        }
        claims.insert(numbers[0]);
    }

    for (auto i = numbers[1]; i < (numbers[1] + numbers[3]); i++)
    {
        for (auto j = numbers[2]; j < (numbers[2] + numbers[4]); j++)
        {
            auto key = i * 10000 + j;
            auto existing = fabric[key];
            if (existing == 0)
            {
                fabric[key] = -numbers[0];
            }
            else if (existing < 0)
            {
                overlapping_count += 1;
                overlapping.insert(-fabric[key]);
                overlapping.insert(numbers[0]);
                fabric[key] = 1;
            }
            else
            {
                overlapping.insert(numbers[0]);
            }
        }
    }
    cout << "Part 1 is " << overlapping_count << endl;
    for (auto claim : claims)
    {
        if (overlapping.count(claim) == 0)
        {
            cout << "Part 2 is " << claim << endl;
        }
    }
}
} // namespace day3