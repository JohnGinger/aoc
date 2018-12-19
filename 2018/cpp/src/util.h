#ifndef UTIL
#define UTIL

#include <iostream>
#include <fstream>
#include <vector>
#include <functional>
#include <list>
using namespace std;

vector<string> get_input_as_vec(string day)
{
    auto lines = vector<string>();
    string line;

    ifstream input("../../data/" + day + ".txt");
    if (input.is_open())
    {
        while (getline(input, line))
        {
            lines.push_back(line);
        }
    }
    else
    {
        cout << "Could not open file" << endl;
    }
    input.close();
    return lines;
}

void time_function(function<void()> func)
{
    const auto start(chrono::high_resolution_clock::now());
    func();
    const auto stop(chrono::high_resolution_clock::now());
    const auto duration_ms(chrono::duration_cast<chrono::duration<double, std::milli>>(stop - start).count());
    printf("Took %6.2lfms", duration_ms);
}

std::string join(const std::list<char> &lst, const std::string &delim)
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
#endif