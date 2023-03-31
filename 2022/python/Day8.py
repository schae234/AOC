#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Mon Jan  2 13:52:31 2023

@author: raegoering

--- Day 7: Treetop Tree House ---

"""

import numpy as np

def tree_array(file):
    row_list = []
    for line in file:
        row = [int(i) for i in str(line.rstrip())]
        row_list.append(row)
    return np.array(row_list)


def is_visible(pos, tree_list):
    # trees on either edge are visible
    if pos == 0 or pos == len(tree_list) - 1:
        return True
    # visible from left
    elif tree_list[pos] > max(tree_list[0:pos]):
        return True
    # visible from right
    elif tree_list[pos] > max(tree_list[pos+1:]):
        return True
    else:
        return False

def viewed_trees(height, tree_list):
    # tree list must be in order they are viewed
    viewed = 0
    for tree in tree_list:
        if height > tree:
            viewed += 1
        else:
            viewed += 1
            break
    return viewed


def scenic_score(x,y,arr):
    # want trees in order they are viewed
    left_trees = viewed_trees(arr[x, y], np.flip(arr[x][0:y]))
    right_trees = viewed_trees(arr[x, y], arr[x][y+1:])
    up_trees = viewed_trees(arr[x, y],  np.flip(arr[:, y][0:x]))
    down_trees = viewed_trees(arr[x, y], arr[:, y][x+1:])
    scenic_score = left_trees * right_trees * up_trees * down_trees
    return scenic_score



# with open('2022/python/Day8_inputexample.txt') as f:
#     grid = tree_array(f)

with open('2022/python/Day8_input.txt') as f:
    grid = tree_array(f)

visible_trees = 0
scenic_trees = []
for ix, iy in np.ndindex(grid.shape):
    if is_visible(ix, grid[iy]) or is_visible(iy, grid[:, ix]):
        visible_trees += 1
    scenic_trees.append(scenic_score(iy, ix, grid))

print(f"there are {visible_trees} visible trees")
print(f"The max scenic score is {max(scenic_trees)}")

