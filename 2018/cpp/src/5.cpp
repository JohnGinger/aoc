#include <iostream>
#include <fstream>
#include <unordered_set>
#include <vector>
#include "util.h"
#include <map>
#include <regex>
#include <unordered_map>
#include <ctime>
#include <numeric>
using namespace std;

namespace day5
{

std::string join(const std::vector<char> &lst, const std::string &delim)
{
    std::string ret;
    for (const auto &s : lst)
    {
        if (!ret.empty())
            ret += delim;
        ret += s;
    }
    return ret;
}

vector<char> react(string word)
{
    vector<char> letters(word.begin(), word.end());
    auto i = 0;
    while (i < letters.size())
    {
        // std::cout << join(letters, "") << endl;
        if (toupper(letters[i]) == toupper(letters[i + 1]) && letters[i] != letters[i + 1])
        {
            letters.erase(letters.begin() + i, letters.begin() + i + 2);
            if (i < 2)
            {
                i = 0;
            }
            else
            {
                i -= 2;
            }
        }
        else
        {
            i += 1;
        }
    }
    return letters;
}

string replace_letter(char letter, vector<char> word)
{
    string output_word;
    for (auto l : word)
    {
        if (l != letter && l != toupper(letter))
        {
            output_word += l;
        }
    }
    return output_word;
}

void run()
{
    auto word = get_input_as_vec("5")[0];
    auto react_first = react(word);
    std::cout << "Part 1 is " << react_first.size() << endl;

    string letters_alphabet = "qwertyuiopasdfghjklzxcvbnm";
    auto min_length = word.size();
    for (auto letter : letters_alphabet)
    {
        auto new_word = replace_letter(letter, react_first);
        auto length_word = react(new_word).size();
        if (length_word < min_length)
        {
            min_length = length_word;
        }
    }
    std::cout << "Part 2 is " << min_length << endl;
}
} // namespace day5