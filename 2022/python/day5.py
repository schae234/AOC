#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Mon Dec 26 18:01:05 2022

@author: raegoering

--- Day 5: Supply Stacks ---

"""
import copy

def read_instructions(i):
    i_list = i.split()
    i_dict = {}
    i_dict["move"] = int(i_list[1])
    i_dict["s1"] = int(i_list[3])-1
    i_dict["s2"] = int(i_list[5])-1
    return i_dict


def StackTops(stack_list):
    tops = ""
    for stack in stack_list:
        tops = tops + stack[-1]
    return tops


def stack_list(stack_image):
    stack_list = [[] for x in range(9)]
    position = 0
    for character in stack_image:
        if character != " " and position in [1, 5, 9, 13, 17, 21, 25, 29, 33]:
            stack = int((position - 1) / 4)
            stack_list[stack].insert(0, character)
        position += 1
    return(stack_list)


def stack_list_insert(sl, sl_line):
    list_num = 0
    for lists in sl_line:
        if lists:
            sl[list_num].insert(0, lists[0])
        list_num += 1
    return sl


# Before I got smart smart, when I was a dum dum:
#   
# ex_stacks = [["Z", "N"],
#              ["M", "C", "D"],
#              ["P"]]

# stacks = [["R", "S", "L", "F", "Q"],
#           ["N", "Z", "Q", "G", "P", "T"],
#           ["S", "M", "Q", "B"],
#           ["T", "G", "Z", "J", "H", "C", "B", "Q"],
#           ["P", "H", "M", "B", "N", "F", "S"],
#           ["P", "C", "Q", "N", "S", "L", "V", "G"],
#           ["W", "C", "F"],
#           ["Q", "H", "G", "Z", "W", "V", "P", "M"],
#           ["G", "Z", "D", "L", "C", "N", "R"]]

# pop removes the top crate (edited in line)
# append adds to the top of the stack (edited in line)

with open('Documents/AOC/2022/python/Day5_input.txt') as f:
    crane = "9001"
    line_num = 0
    stacks = [[] for x in range(9)]
    for line in f:
        if line_num < 8:
            sl = stack_list(line.rstrip())
            print(line.rstrip())
            starting_stacks = stack_list_insert(stacks, sl)
            stacks9000 = copy.deepcopy(starting_stacks)
            stacks9001 = copy.deepcopy(starting_stacks)
        # skip empty line and line with stack numbers
        elif line_num in [8, 9]:
            pass
        else:
            i = read_instructions(line)
            n = 1
            removed9001 = []
            while n <= i["move"]:
                removed9000 = stacks9000[i["s1"]].pop()
                stacks9000[i["s2"]].append(removed9000)
                removed9001.insert(0, stacks9001[i["s1"]].pop())
                n += 1
            stacks9001[i["s2"]] = stacks9001[i["s2"]] + removed9001
        line_num += 1
    print("Convert this image to a list of lists printed nicely below: ")
    for starting_stack in starting_stacks:
        print(starting_stack)
    print("Now following all the instructions we end up with:")
    for stack9000 in stacks9000:
        print(stack9000)
    print(f"And the puzzle solution is: {StackTops(stacks9000)}")
    print("When we use a crane 9001, we end up with:")
    for stack9001 in stacks9001:
        print(stack9001)
    print(f"And the puzzle solution is: {StackTops(stacks9001)}")
