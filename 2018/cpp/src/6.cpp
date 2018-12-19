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

namespace day6
{
bool DISPLAY = true;
struct Point
{
    int id;
    int x;
    int y;
};

int getClosestPoint(vector<Point> points, int x, int y, int min_distance = 10000000)
{
    auto closest_point = -1;

    if (points.size() == 0)
    {
        return 0;
    }

    for (auto point : points)
    {
        auto distance = abs(point.x - x) + abs(point.y - y);
        if (distance == 0)
        {
            return point.id;
        }
        if (distance < min_distance)
        {
            min_distance = distance;
            closest_point = point.id;
        }
        else if (distance == min_distance)
        {
            auto new_points = vector<Point>();
            for (auto p : points)
            {
                if (p.id != point.id && p.id != closest_point)
                {
                    new_points.push_back(p);
                }
            }
            return getClosestPoint(new_points, x, y, min_distance);
        }
    }

    if (closest_point == -1)
    {
        return 0;
    }

    return closest_point;
}

int getPointSum(vector<Point> points, int x, int y)
{
    auto sum = 0;

    for (auto point : points)
    {
        sum += abs(point.x - x) + abs(point.y - y);
    }
    return sum;
}

void run()
{
    std::regex number_regex("([0-9]+), ([0-9]+)");
    auto words = get_input_as_vec("6");
    auto points = vector<Point>();

    auto point_num = 1;
    for (auto word : words)
    {

        std::smatch matches;
        if (regex_search(word, matches, number_regex))
        {

            points.push_back({point_num, stoi(matches[1]), stoi(matches[2])});
            point_num += 1;
        }
    }
    auto min_x = 1000000;
    auto max_x = 0;
    auto min_y = 1000000;
    auto max_y = 0;

    for (auto point : points)
    {
        auto x = point.x;
        auto y = point.y;
        max_x = max(x, max_x);
        max_y = max(y, max_y);
        min_x = min(x, min_x);
        min_y = min(y, min_y);
    }

    auto infinite_points = unordered_set<int>();
    auto areas = unordered_map<int, int>();
    auto safe_area = 0;
    auto letters = ".abcdefghijklmnopqrstuvqxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!@£$^&*()";
    for (auto y = min_y; y <= max_y; y++)
    {
        for (auto x = min_x; x <= max_x; x++)
        {
            auto closest_point = getClosestPoint(points, x, y);
            auto point_sum = getPointSum(points, x, y);

            if (point_sum < 10000)
            {
                if (DISPLAY)
                {
                    std::cout << '#';
                }
                safe_area += 1;
            }
            else
            {
                if (DISPLAY)
                {
                    std::cout << '.';
                }
            }

            if (y == min_y || y == max_y || x == min_x || x == max_x)
            {
                infinite_points.insert(closest_point);
            }
            else
            {
                areas[closest_point] += 1;
            }
            if (DISPLAY)
            {
                std::cout << letters[closest_point];
            }
        }
        if (DISPLAY)
        {
            std::cout << endl;
        }
    }

    auto biggest_area = 0;
    for (auto area : areas)
    {
        if (area.first == 0)
        {
            continue;
        }
        auto point = points[area.first - 1];
        if (!infinite_points.count(point.id))
        {
            biggest_area = max(biggest_area, area.second);
        }
    }

    std::cout << "Part 1 is " << biggest_area << endl;
    std::cout << "Part 2 is " << safe_area << endl;
}
} // namespace day6