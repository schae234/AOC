#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Wed Jan  4 17:49:20 2023

@author: raegoering

--- Day 10: Cathode-Ray Tube ---

"""

def execute_program(file):
    cycle = 0
    x = 1
    cycle_x = {}
    #cycle_x[cycle] = x
    for line in file:
        line = line.strip()
        if line == "noop":
            cycle += 1
            cycle_x[cycle] = x
        elif line.startswith("addx"):
            value = int(line.split(" ")[1])
            cycle += 1
            cycle_x[cycle] = x
            cycle += 1
            cycle_x[cycle] = x
            x = x + value
    return cycle_x

def get_ss(cycle_x):
    ss_sum = 0
    ss_list = []
    for cycle in cycle_x.keys():
        if cycle in [20, 60, 100, 140, 180, 220]:
            ss = cycle_x[cycle] * cycle
            ss_sum = ss_sum + ss
            ss_list.append(ss)
    return ss_sum
    
def sprite_string(x):
    if x == 0:
        sprite = "##" + "." * 38
    elif x == 39:
        sprite = "." * 38 + "##"
    else:
        sprite = "."*(x-1)+"###"+"."*(38-x)
    return sprite
    

def draw_pixels(cycle, cycle_x):
    sprite = sprite_string(cycle_x[cycle])
    pos = cycle - 1 - 40*(int(cycle/40))
    if sprite[pos] == "#":
        pixel = "#"
    elif sprite[pos] == ".":
        pixel = "."
    return pixel

    
with open('2022/python/Day10_input.txt') as f:
    program = execute_program(f)
print(f"The sum of signal strengths is {get_ss(program)}.")

pixel_line_num = 0
pixels = ""
for cycle in program.keys():
    if int((cycle-1)/40) == pixel_line_num:
        pixels = pixels + draw_pixels(cycle, program)
        #print(cycle, program[cycle])
    else:
        pixels = pixels + "\n" + draw_pixels(cycle, program)
        pixel_line_num += 1
print(pixels)
