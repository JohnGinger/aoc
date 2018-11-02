from __future__ import print_function
import sys

def isWall(x, y, favNumber):
    if x < 0 or y < 0:
        return True
    score = x * x + 3 * x + 2 * x * y + y + y * y + favNumber
    binary = list(bin(score))[2:]
    return len(filter(lambda x: x == '1', binary)) % 2 != 0

def get_neighbours(point):
    (x, y) = point
    neighbours = []
    if not isWall((x - 1), y, 1350):
        neighbours.append((x - 1, y))
    if not isWall(x + 1, y, 1350):
        neighbours.append((x + 1, y))
    if not isWall(x, y - 1, 1350):
        neighbours.append((x, y - 1))
    if not isWall(x, y + 1, 1350):
        neighbours.append((x, y + 1))
    return neighbours
  
def a_star_search(start, goal):
    closed_set = []
    open_set = [start]
    came_from = {}
    cost_to_this_point = {}
    came_from[start] = None
    cost_to_this_point[start] = 0
    heuristic_score = {}
    heuristic_score[start] = heuristic(start, goal)
    
    current = {}
    while open_set:
        current = sorted(open_set, key=lambda x: heuristic_score[x])[0]

        open_set.remove(current)
        closed_set.append(current)
        
        for next in get_neighbours(current):
            if next in closed_set:
                continue
            new_cost = cost_to_this_point[current] + 1
            if new_cost > 50:
                continue
            elif next not in open_set:
                open_set.append(next)
                came_from[next] = current
                cost_to_this_point[next] = new_cost
                heuristic_score[next] = new_cost + heuristic(next, goal)
    return came_from, cost_to_this_point, current

def heuristic(a, b):
    (x1, y1) = a
    (x2, y2) = b
    return abs(x1 - x2) + abs(y1 - y2)


def reconstruct(came_from, current):
    path = [current]
    while current in came_from:
        current = came_from[current]
        path.append(current)
    return path

(came_from, cost_to_this_point, current) = a_star_search((1,1), (31, 39))
path = reconstruct(came_from, current)
print(len(came_from))

