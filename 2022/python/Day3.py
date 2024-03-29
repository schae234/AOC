#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Sun Dec 25 13:38:52 2022

@author: raegoering

--- Day 3: Rucksack Reorganization ---

"""
import string
from itertools import islice

def split_contents(contents_string, compartment):
    """

    Parameters
    ----------
    contents_string : str
    compartment : int

    Returns
    -------
    contents : str

        This function splits a load into either of its two compartments

    """
    if compartment == 1:
        contents = contents_string[0:int(len(contents_string)/2)]
    elif compartment == 2:
        contents = contents_string[int(len(contents_string)/2):len(contents_string)]
    return contents


def find_missorted(contents1, contents2):
    """

    Parameters
    ----------
    contents1 : str
    contents2 : str

    Returns
    -------
    missorted : str

        This function identifies a shared element in two strings, or the missorted item.

    """
    for contents in contents1:
        if contents in contents2:
            missorted = contents
            return missorted


def get_priority(missorted):
    """

    Parameters
    ----------
    missorted : str

    Returns
    -------
    priority : int

        This function calculates the priority of the missorted item

    """
    keys = list(string.ascii_lowercase[:26] + string.ascii_uppercase[:26])
    values = list(range(1, 58))
    priorities = dict(zip(keys, values))
    return priorities[missorted]


def find_badge(group_list):
    """

    Parameters
    ----------
    group_list : list of length 3 containing item strings of each elf in group

    Returns
    -------
    badge : str

        This function finds the badge shared by each elf in the group

    """
    contents1 = group_list[0]
    contents2 = group_list[1]
    contents3 = group_list[2]
    for contents in contents1:
        if contents in contents2 and contents in contents3:
            badge = contents
            return badge


priority_sum = 0

with open('Documents/AOC/2022/python/Day3_input.txt') as f:
    for line in f:
        comp1 = split_contents(line, 1)
        comp2 = split_contents(line, 2)
        missorted = find_missorted(comp1,comp2)
        priority_sum += get_priority(missorted)

print(f"The sum of the missorted items priorities is {priority_sum}")

priority_sum = 0
n = 0

with open('Documents/AOC/2022/python/Day3_input.txt') as f:
    while True:
        try:
            group = [next(f).rstrip() for x in range(3)]
            badge = find_badge(group)
            priority_sum += get_priority(badge)
        except StopIteration:
            break

print(f"The sum of the group badge priorities is {priority_sum}")
