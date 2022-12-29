#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Mon Dec 26 11:30:48 2022

@author: raegoering

--- Day 4: Camp Cleanup ---

"""

def range2list(num_range):
    """

    Parameters
    ----------
    num_range : str describing a range ex. "2-4"

    Returns
    -------
    num_list : list of integers in that range

        This function converts a string describing a range to an integer list.
    """
    start = int(num_range.split(sep="-")[0])
    end = int(num_range.split(sep="-")[1])
    num_list = list(range(start, end + 1))
    return num_list
    
def contained_within(num_list1, num_list2):
    """

    Parameters
    ----------
    num_list1 : list of integer chores of first elf
    num_list2 : list of integer chores of second elf

    Returns
    -------
    bool

        This function determines if a chore list is contained within another

    """
    if num_list1[0] <= num_list2[0] and num_list1[-1] >= num_list2[-1]:
        return True
    elif num_list2[0] <= num_list1[0] and num_list2[-1] >= num_list1[-1]:
        return True
    else:
        return False
    
def any_overlap(num_list1, num_list2):
    """

    Parameters
    ----------
    num_list1 : list of integer chores of first elf
    num_list2 : list of integer chores of second elf

    Returns
    -------
    bool

        This function determines if a chore list overlaps with another

    """
    for nums in num_list1:
        if nums in num_list2:
            return True
    else:
        return False

full_overlap = 0
overlap = 0

with open('Documents/AOC/2022/python/Day4_input.txt') as f:
    for line in f:
        pairs = line.rstrip().split(sep=",")
        p1 = range2list(pairs[0])
        p2 = range2list(pairs[1])
        if contained_within(p1, p2):
            full_overlap += 1
        elif any_overlap(p1, p2):
            overlap += 1
    total_overlap = full_overlap + overlap
    print(f"Chores with full overlap between paired elves: {full_overlap}")
    print(f"Chores with any overlap between paired elves: {total_overlap}")
    