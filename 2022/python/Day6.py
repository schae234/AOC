#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Thu Dec 29 11:40:03 2022

@author: raegoering

--- Day 6: Tuning Trouble ---

"""

def unique_marker(marker, marker_type):
    """

    Parameters
    ----------
    marker : str
    marker_type : str: "packet" or "message"

    Returns
    -------
    bool
        This function evaluates if a marker contains unique characters.
        start of packet markers must have 4 unique characters.
        start of message markers must have 14 unique characters.

    """
    marker_set = ''.join(set(marker))
    if marker_type == "packet":
        if len(marker_set) == 4:
            return True
        else:
            return False
    elif marker_type == "message":
        if len(marker_set) == 14:
            return True
        else:
            return False


def find_marker_position(datastream):
    """

    Parameters
    ----------
    datastream : str containing transmission with packet and message markers

    Returns
    -------
    None.
        This function will print the positions of packet and message markers

    """
    p = 0
    packet_found = False
    message_found = False
    while not packet_found or not message_found:
        if not packet_found and unique_marker(datastream[p:p+4], "packet"):
            print(f"packet starts at {p + 4}")
            packet_found = True
        elif unique_marker(datastream[p:p+14], "message"):
            print(f"message starts at {p + 14}")
            message_found = True
        p += 1


transmission = ""
with open('Day6_input.txt') as f:
    for line in f:
        transmission = transmission + line.rstrip()
find_marker_position(transmission)
