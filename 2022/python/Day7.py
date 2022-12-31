#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Thu Dec 29 13:05:57 2022

@author: raegoering

--- Day 7: No Space Left On Device ---

"""

def get_dir_dict(file):
    dir_dict = {}
    for line in file:
        line = line.rstrip()
        if line.startswith("$ cd"):
            cmd = line[5:]
            if cmd == "/":
                current_dir = "/"
            elif cmd != "..":
                if current_dir == "/":
                    current_dir = current_dir + cmd
                else:
                    current_dir = current_dir + "/" + cmd
            elif cmd == "..":
                parent = current_dir.split(sep="/")[:-1]
                if len(parent) == 1 and parent[0] == "":
                    parent = "/"
                current_dir = "/".join(parent)
            if current_dir not in dir_dict:
                dir_dict[current_dir] = []
        elif not line.startswith("$"):
            if line.startswith("dir"):
                size = 0
                if current_dir == "/":
                    name = current_dir + line[4:]
                else:
                    name = current_dir + "/" + line[4:]
            else:
                size = line.split(sep=" ")[0]
                if current_dir == "/":
                    name = current_dir + line.split(sep=" ")[1]
                else:
                    name = current_dir + "/" + line.split(sep=" ")[1]
            dir_dict[current_dir].append(tuple([name, size]))
    return dir_dict


def get_size(dir_contents, size_dict):
    dirs = []
    size = 0
    for item in dir_contents:
        # if item is a directory
        if item[1] == 0:
            dir_name = item[0]
            if dir_name in size_dict and type(size_dict[dir_name]) == int:
                size = size + size_dict[dir_name]
            else:
                dirs.append(item)
        else:
            size = size + int(item[1])
    dirs.append(size)
    return dirs


def all_int(d):
    # This function evaluates if a dictionary only contains integers
    if len(d.values()) == 0:
        return False
    all_int = all([type(i) == int for i in d.values()])
    return all_int


# with open('2022/python/Day7_inputexample.txt') as f:
#     directory_dict = get_dir_dict(f)
    
with open('2022/python/Day7_input.txt') as f:
    directory_dict = get_dir_dict(f)

# print directory nicely
print("directories:")
for key in directory_dict.keys():
    print(key, directory_dict[key])

# calculate directory sizes
dir_size_dict = {}
while not all_int(dir_size_dict):
    for key in directory_dict.keys():
        size_list = get_size(directory_dict[key], dir_size_dict)
        if len(size_list) == 1 and type(size_list[0]) == int:
            dir_size_dict[key] = get_size(directory_dict[key], dir_size_dict)[0]
        else:
            dir_size_dict[key] = get_size(directory_dict[key], dir_size_dict)

# print directory sizes nicely
print("\ndirectory sizes:")
for key in dir_size_dict.keys():
    print(key, dir_size_dict[key])

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

# the correct answer was 1432936

#part2
total_space = 70000000
available_space = 70000000-dir_size_dict["/"]
update_space = 30000000
needed_space = 30000000-available_space

print(f"\nWe have {total_space} space total")
print(f"The root directory takes up {dir_size_dict['/']}")
print(f"So we have {available_space} free space available")
print(f"but to install our 30,000,000 new update, we still need {needed_space}")

sorted_size_dict = dict(sorted(dir_size_dict.items(), key=lambda directory: directory[1]))
for key in sorted_size_dict:
    if sorted_size_dict[key] > needed_space:
        print(f"we can delete {key} with a size of {sorted_size_dict[key]}")
        break