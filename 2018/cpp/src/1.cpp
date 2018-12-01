#include <iostream>
#include <fstream>
#include <unordered_set>
#include <vector>

using namespace std;

namespace day1
{
void run()
{
    string line;
    int sum = 0;
    auto reached = unordered_set<int>();
    auto reached_duplicate = false;
    auto first_loop = true;

    auto lines = get_input_as_vec("1");

    while (!reached_duplicate)
    {
        for (auto it = lines.begin(); it < lines.end(); it++)
        {
            sum += stoi(*it);
            if (reached.count(sum) == 1 && !reached_duplicate)
            {
                cout << "Part 2 is " << sum << endl;
                reached_duplicate = true;
            }
            else
            {
                reached.insert(sum);
            }
        }
        if (first_loop)
        {
            cout << "Part 1 is " << sum << endl;
            first_loop = false;
        }
    }
}
}