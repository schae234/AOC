#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Fri Dec 23 10:28:06 2022

@author: raegoering

"""
from statistics import mean

def get_keys(d, value_list):
    # This function returns keys in a dictionary matching values in a list
    # It should return keys matching the order of the value list
    keys = []
    for value in value_list:
        key = ([i for i in d if d[i] == value])
        keys.append(key[0])
    return keys

"""
--- Day 1: Calorie Counting ---

"""

# first we will build a dictionary of elves and how many calories they are carrying.

elf_dict = {}
elf_counter = 0
# because the file does not begin with an empty line, we must initiate the first elf in the dictionary
elf_dict[elf_counter] = 0

with open('Documents/AOC/2022/python/Day1_input.txt') as f:
    for line in f:
        # an empty line indicates a new elf
        if line.rstrip() == "":
            elf_counter += 1
            elf_dict[elf_counter] = 0
        else:
            # elves can carry several items but we only care about total calories
            elf_dict[elf_counter] += int(line)

# what was the heaviest load carried?
max_calories = max(elf_dict.values())
# which elf carried that load?
max_elf = get_keys(elf_dict, [max_calories])

# what else can we know about these elves?
print(f"There are {len(elf_dict)} elves.")
print(f"Carrying a total of {sum(elf_dict.values())} calories.")
print(f"The average elf carries {round(mean(elf_dict.values()),0)} calories.")
print(f"But, elf {max_elf[0]} carried the most, {max_calories} calories")
print(f"Thats {round(max_calories / mean(elf_dict.values()),2)} times more than the average elf.")
print(f"Way to go elf {max_elf[0]}!\n")

# who are the 3 strongest elves?
# lets sort the dictionary but maintain our elves names
sorted_elf_dict = dict(sorted(elf_dict.items(), reverse=True, key=lambda elf: elf[1]))
top3_loads = list(sorted_elf_dict.values())[0:3]
top3_elves = get_keys(sorted_elf_dict, top3_loads)

print(f"The strongest 3 elves are elf {top3_elves[0]}, elf {top3_elves[1]}, and elf {top3_elves[2]}")
print(f"In total they are carrying {sum(top3_loads)} calories.")
