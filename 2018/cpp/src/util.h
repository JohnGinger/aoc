#ifndef UTIL
#define UTIL

#include <iostream>
#include <fstream>
#include <vector>
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

void time_function(string name, void (*f)())
{
    const auto start(chrono::high_resolution_clock::now());
    (*f)();
    const auto stop(chrono::high_resolution_clock::now());
    const auto duration_ms(chrono::duration_cast<chrono::duration<double, std::milli>>(stop - start).count());
    printf("%s took %6.2lfms\n", name.c_str(), duration_ms);
}
#endif