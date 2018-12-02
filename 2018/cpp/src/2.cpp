#include <iostream>
#include <fstream>
#include <unordered_set>
#include <vector>
#include "util.h"
#include <map>

using namespace std;

namespace day2
{

bool has_repeated(map<char, int> counts, int to_check)
{
    for (auto it = counts.begin(); it != counts.end(); ++it)
    {
        if (it->second == to_check)
        {
            return true;
        }
    }
    return false;
}

string compare_words(string word1, string word2)
{
    string common;
    for (auto i = 0; i < word1.length(); i++)
    {
        if (word1[i] == word2[i])
        {
            common += word1[i];
        }
    }
    return common;
}

void run()
{
    auto lines = get_input_as_vec("2");
    auto two_count = 0;
    auto three_count = 0;

    for (auto it = lines.begin(); it < lines.end(); it++)
    {
        map<char, int> counts;

        auto line = *it;
        for (auto letter = line.begin(); letter != line.end(); ++letter)
        {
            counts[*letter] += 1;
        }

        if (has_repeated(counts, 2))
        {
            two_count += 1;
        }
        if (has_repeated(counts, 3))
        {
            three_count += 1;
        }
    }
    cout << "Part 1 is " << two_count * three_count << endl;

    string last_line;
    auto common_letters_length = 0;
    string best_common_letters;
    sort(lines.begin(), lines.end());
    for (auto line : lines)
    {
        auto common_letters = compare_words(last_line, line);
        last_line = line;
        if (common_letters.length() > common_letters_length)
        {
            common_letters_length = common_letters.length();
            best_common_letters = common_letters;
        }
    }
    std::cout << "Part 2 is " << best_common_letters << endl;
}

} // namespace day2