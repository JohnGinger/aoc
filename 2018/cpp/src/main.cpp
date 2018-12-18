#include <functional>
#include "util.h"
#include "1.cpp"
#include "2.cpp"
#include "3.cpp"
#include "4.cpp"
#include "5.cpp"

int main()
{
    vector<function<void()>> days = {day1::run, day2::run, day3::run, day4::run, day5::run};
    auto developing = true;
    if (developing)
    {
        cout << "Developing..." << endl;
        time_function(days[days.size() - 1]);
        return 0;
    }

    auto day_num = 1;
    for (auto day : days)
    {
        cout << "Day " << day_num << endl;
        time_function(day);
        cout << "\n"
             << endl;
        day_num += 1;
    }
    return 0;
}