#include <iostream>
#include <fstream>
#include <unordered_set>
#include <vector>
#include "util.h"
#include <map>
#include <regex>
#include <unordered_map>
#include <ctime>

using namespace std;

namespace day4
{

int days_till_start_of_month(int month)
{
    auto m = (month + 9) % 12;
    return (m * 306 + 5) / 10;
}

struct Guard
{
    int time;
    bool wake;
    bool sleep;
    bool start;
    int id;
};

bool compareByTime(const Guard &a, const Guard &b)
{
    return a.time < b.time;
}

bool sort_map(const pair<int, int> &p1, const pair<int, int> &p2) { return p1.second < p2.second; }

map<int, int> get_minutes_alseep(map<int, int> common_asleep, int start, int end)
{
    for (auto i = start; i < end; i++)
    {
        common_asleep[i % 60] += 1;
    }
    return common_asleep;
}

int get_part_two(map<int, map<int, int>> common_asleep)
{
    auto most_asleep = 0;
    auto part2_guard_id = 0;
    auto part2_minute_asleep = 0;
    for (auto const &guard : common_asleep)
    {
        auto guard_id = guard.first;
        auto sleeping_pattern = common_asleep[guard_id];
        auto minute_most_asleep = std::max_element(sleeping_pattern.begin(), sleeping_pattern.end(), sort_map);
        auto minute = minute_most_asleep->first;
        auto time_asleep = minute_most_asleep->second;
        if (time_asleep > most_asleep)
        {
            part2_guard_id = guard_id;
            part2_minute_asleep = minute;
            most_asleep = time_asleep;
        }
    }
    return part2_guard_id * part2_minute_asleep;
}

void run()
{
    // [1518-06-05 00:46] falls asleep
    // [1518-11-10 23:52] Guard #881 begins shift
    std::regex line_regex("\\[([0-9]+)-([0-9]+)-([0-9]+) ([0-9]+):([0-9]+)\\] ([f|w|G])(.*)");
    std::regex number_regex("([0-9]+)");

    auto lines = get_input_as_vec("4");
    auto guards = vector<Guard>();
    for (auto line : lines)
    {
        std::smatch matches;
        if (regex_search(line, matches, line_regex))
        {
            if (matches.size() != 8)
            {
                std::cout << "Couldn't parse line!" << matches.size() << endl;
            }
            else
            {
                auto time = stoi(matches[5].str()) +
                            (stoi(matches[4]) * 60) +
                            (stoi(matches[3]) * 60 * 24) +
                            (days_till_start_of_month(stoi(matches[2])) * 60 * 24) +
                            (stoi(matches[1]) * 60 * 24 * 365);

                auto start = matches[6].str() == "G";
                auto wake = matches[6].str() == "w";
                auto sleep = matches[6].str() == "f";

                auto guard_id = -1;
                if (start)
                {
                    smatch guard_id_match;
                    auto match_string = matches[7].str();
                    regex_search(match_string, guard_id_match, number_regex);
                    guard_id = stoi(guard_id_match[1]);
                }
                auto new_guard = Guard{time, wake, sleep, start, guard_id};
                guards.push_back(new_guard);
            }
        }
        else
        {
            std::cout << "Match not found\n";
        }
    }

    sort(guards.begin(), guards.end(), compareByTime);
    auto minutes_asleep = map<int, int>();
    auto common_asleep = map<int, map<int, int>>();

    auto asleep = false;
    auto last_guard_id = -1;
    auto fall_asleep_time = 0;

    for (auto guard : guards)
    {

        if (guard.sleep)
        {
            asleep = true;
            fall_asleep_time = guard.time;
        }
        else if (guard.wake)
        {
            if (asleep)
            {
                asleep = false;
                minutes_asleep[last_guard_id] += guard.time - fall_asleep_time;
                common_asleep[last_guard_id] = get_minutes_alseep(common_asleep[last_guard_id], fall_asleep_time, guard.time);
            }
        }
        else if (guard.start)
        {
            last_guard_id = guard.id;
            asleep = false;
        }
    }
    auto guard_most_time_asleep = std::max_element(minutes_asleep.begin(), minutes_asleep.end(), sort_map);
    auto sleepy_guard_id = guard_most_time_asleep->first;
    auto minutes_that_guard_most_sleeps = common_asleep[sleepy_guard_id];
    auto minute_most_asleep = std::max_element(minutes_that_guard_most_sleeps.begin(), minutes_that_guard_most_sleeps.end(), sort_map);

    std::cout << "Part 1 is " << sleepy_guard_id * minute_most_asleep->first << endl;
    auto part2_guard_id = get_part_two(common_asleep);
    std::cout << "Part 2 is " << part2_guard_id << endl;
}

} // namespace day4