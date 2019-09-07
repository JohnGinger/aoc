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
using namespace std;

namespace day7
{

unordered_map<char, int> NODE_TIMES =
    {
    {'A', 61},
    {'B', 62},
    {'C', 63},
    {'D', 64},
    {'E', 65},
    {'F', 66},
    {'G', 67},
    {'H', 68},
    {'I', 69},
    {'J', 70},
    {'K', 71},
    {'L', 72},
    {'M', 73},
    {'N', 74},
    {'O', 75},
    {'P', 76},
    {'Q', 77},
    {'R', 78},
    {'S', 79},
    {'T', 80},
    {'U', 81},
    {'V', 82},
    {'W', 83},
    {'X', 84},
    {'Y', 85},
    {'Z', 86},
};

struct Edge
{
    char from;
    char to;
};

struct Node
{
    unordered_set<char> parents;
    unordered_set<char> children;
    int time_remaining = -1;
};

Edge parse_input_line(string line)
{
    std::regex line_regex("Step ([A-Z]) must be finished before step ([A-Z]) can begin.");
    std::smatch matches;
    if (regex_search(line, matches, line_regex))
    {
        auto edge = Edge{};
        string from = matches[1];
        string to = matches[2];
        edge.from = from[0];
        edge.to = to[0];
        return edge;
    }
    else
    {
        throw std::invalid_argument("Could not parse line");
    }
}

set<char> get_nodes_without_parents(unordered_map<char, Node> nodes) {
    set<char> nodes_with_parents; // Set provides alphabetical sorting
    for (auto node : nodes)
    {
        if (node.second.parents.size() == 0)
        {
            nodes_with_parents.insert(node.first);
        }
    }

    if (nodes_with_parents.size() > 0)
    {
        return nodes_with_parents;
    }
    throw std::invalid_argument("Graph not complete");
}

char get_node_without_parents(unordered_map<char, Node> nodes)
{
    return *get_nodes_without_parents(nodes).begin();
};

void remove_node_from_nodes(unordered_map<char, Node> &nodes, char node_id)
{
    Node node = nodes[node_id];
    for (auto child_node_id : node.children)
    {
        nodes[child_node_id].parents.erase(node_id);
    }
    nodes.erase(node_id);
};

void debug_print_nodes(unordered_map<char, Node> &nodes)
{
    for (auto node : nodes)
    {
        cout << "Node: " << node.first;
        cout << " has children " << string(node.second.children.begin(), node.second.children.end());
        cout << " and parents " << string(node.second.parents.begin(), node.second.parents.end()) << endl;
    }
};

unordered_map<char, Node> get_nodes_from_lines(vector<string> &lines)
{
    auto nodes = unordered_map<char, Node>();
    for (auto line : lines)
    {
        auto edge = day7::parse_input_line(line);
        nodes[edge.from].children.insert(edge.to);
        nodes[edge.to].parents.insert(edge.from);
    }
    return nodes;
};

void decrement_active_nodes(unordered_map<char, Node> &nodes)
{
    for (auto &node : nodes)
    {
        if (node.second.time_remaining > 0)
        {
            node.second.time_remaining -= 1;
        }
    }
};

void remove_finished_nodes(unordered_map<char, Node> &nodes)
{
    vector<char> ids_to_remove;
    for (auto node : nodes)
    {
        if (node.second.time_remaining == 0)
        {
            ids_to_remove.push_back(node.first);
        }
    }
    for (auto node_id : ids_to_remove){
        remove_node_from_nodes(nodes, node_id);
    }
};

int get_number_of_currently_active_nodes(unordered_map<char, Node> &nodes){
    auto number_active =0;
        for (auto node : nodes)
    {
        if (node.second.time_remaining > 0)
        {
            number_active +=1;
        }
    }
    return number_active;
}

vector<char> get_next_nodes_to_work_on(unordered_map<char, Node> &nodes, int workers){
    auto nodes_without_parents = get_nodes_without_parents(nodes);
    auto currently_active = get_number_of_currently_active_nodes(nodes);
    auto can_start = workers - currently_active;
    vector<char> next_nodes;
    vector<char> possible_nodes(nodes_without_parents.begin(), nodes_without_parents.end());
    for (int i =0; i < possible_nodes.size() && next_nodes.size() < can_start; i++){
        auto node_id = possible_nodes[i];
        if (nodes[node_id].time_remaining == -1){
            next_nodes.push_back(node_id);
        }
    }
    return next_nodes;
};

void start_working_on_nodes(unordered_map<char, Node> &nodes, vector<char> node_ids){
    for(auto node_id: node_ids){
        nodes[node_id].time_remaining = NODE_TIMES[node_id];
    }
}

void debug_print_parallel(unordered_map<char, Node> &nodes, int time_taken, vector<char> node_ids){
            cout << "Time is " << time_taken << endl;
        if (node_ids.size()){
         cout << " starting "<< string(node_ids.begin(), node_ids.end()) << endl;
        }
        cout << "Working on ";
        for (auto node: nodes){
            if (node.second.time_remaining > 0){
            cout << node.first << "(" << node.second.time_remaining << ") " ;
            }
        }
        cout << endl;
}

void run()
{
    auto lines = get_input_as_vec("7");
    auto nodes = get_nodes_from_lines(lines);
    vector<char> build_order;
    while (nodes.size() > 0)
    {
        auto node_id = get_node_without_parents(nodes);
        remove_node_from_nodes(nodes, node_id);
        build_order.push_back(node_id);
    }
    cout << "Part 1 is " << string(build_order.begin(), build_order.end()) << endl;

    nodes = get_nodes_from_lines(lines);

    auto time_taken = 0;
    vector<char> active_nodes;
    auto number_of_workers = 5;
    while (nodes.size() > 0 && time_taken < 1000)
    {
  
        auto node_ids = get_next_nodes_to_work_on(nodes, number_of_workers);
        start_working_on_nodes(nodes, node_ids);
        decrement_active_nodes(nodes);
        remove_finished_nodes(nodes);
        time_taken += 1;
    }
    cout << "Part 2 is " << time_taken << " seconds" << endl;
}
} // namespace day7