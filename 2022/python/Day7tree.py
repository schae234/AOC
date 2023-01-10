#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Fri Dec 30 12:39:03 2022

@author: raegoering

--- Day 7: No Space Left On Device ---
--- but with a tree ---

"""
from queue import Queue

class Node:
    def __init__(self, name, parent, size=0):
        self.name = name
        self.parent = parent
        self.children = []
        self.size = size

    def __str__(self, level=0):
        ret = "\t"*level+repr(self.name)+"\n"
        for child in self.children:
            ret += child.__str__(level+1)
        return ret

    def __repr__(self):
        return self.name

def make_tree(file):
    root = Node("/", None)
    for line in file:
        line = line.rstrip()
        if line.startswith("$ cd"):
            cmd = line[5:]
            if cmd == "/":
                current_dir = root
            elif cmd != "..":
                child = None
                for c in current_dir.children:
                    if c.name == cmd:
                        child = c
                if child == None:
                    child = Node(cmd, current_dir)
                current_dir = child
            elif cmd == "..":
                current_dir = current_dir.parent
        elif not line.startswith("$"):
            if line.startswith("dir"):
                current_dir.children.append(Node(line[4:], current_dir))
            else:
                size = line.split(sep=" ")[0]
                name = line.split(sep=" ")[1]
                current_dir.children.append(Node(name, current_dir, size))
    return root


def get_size(node, size_dict):
    dirs = []
    size = 0
    for c in node.children:
        # if child is a directory
        if c.size == 0:
            if c in size_dict and type(size_dict[c]) == int:
                size = size + size_dict[c]
            else:
                dirs.append(c)
        else:
            size = size + int(c.size)
    dirs.append(size)
    return dirs


def all_int(d):
    # This function evaluates if a dictionary only contains integers
    if len(d.values()) == 0:
        return False
    all_int = all([type(i) == int for i in d.values()])
    return all_int


def refine_size_dict(node):
    size_list = get_size(node, dir_size_dict)
    if len(size_list) == 1 and type(size_list[0]) == int:
        dir_size_dict[node] = get_size(node, dir_size_dict)[0]
    else:
        dir_size_dict[node] = get_size(node, dir_size_dict)
            
# with open('2022/python/Day7_inputexample.txt') as f:
#     root = make_tree(f)

with open('2022/python/Day7_input.txt') as f:
    root = make_tree(f)

#print(f"directories are\n {root}")

# get sizes from tree
dir_size_dict = {}
node_list = [root]
while not all_int(dir_size_dict):
    for node in node_list:
        refine_size_dict(node)
        for c in node.children:
            if c.children:
                if c not in node_list:
                    node_list.append(c)
                refine_size_dict(c)
                    
# print directory sizes nicely
print("\ndirectory sizes:")
for key in dir_size_dict.keys():
    print(key.name, dir_size_dict[key])

# sum the sizes of directories containing less than 100000
dir_num = 0
total_size = 0
dirs = []
for key in dir_size_dict.keys():
    if dir_size_dict[key] <= 100000:
        dir_num += 1
        total_size = total_size + dir_size_dict[key]
        dirs.append(key)
print(f"\n{dir_num} directories, specifically {dirs} are smaller than 100,000 containing a total of {total_size}")

#part2
total_space = 70000000
available_space = 70000000-dir_size_dict[root]
update_space = 30000000
needed_space = 30000000-available_space

print(f"\nWe have {total_space} space total")
print(f"The root directory takes up {dir_size_dict[root]}")
print(f"So we have {available_space} free space available")
print(f"but to install our 30,000,000 new update, we still need {needed_space}")

sorted_size_dict = dict(sorted(dir_size_dict.items(), key=lambda directory: directory[1]))
for key in sorted_size_dict:
    if sorted_size_dict[key] > needed_space:
        print(f"we can delete {key.name} with a size of {sorted_size_dict[key]}")
        break
