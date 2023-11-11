#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Tue Jan  3 16:14:16 2023

@author: raegoering

--- Day 9: Rope Bridge ---

"""

class point:
    def __init__(self, x=0, y=0):
        self.x = x
        self.y = y
        self.name = ('({0}, {1})'.format(self.x, self.y))

    def __sub__(self, other): 
        dx = self.x - other.x
        dy = self.y - other.y
        return point(dx, dy)
    
    def __add__(self, other):
        dx = self.x + other.x
        dy = self.y + other.y
        return point(dx, dy)
    
    def __contains__(self, point_list):
        if self.name in point_list:
            return True
        else:
            return False

    def __str__(self):
        return ('({0}, {1})'.format(self.x, self.y))
 
    def __repr__(self):
        return ('({0}, {1})'.format(self.x, self.y))


def move(direction, pos):
    if direction == "R":
        pos.x = pos.x + 1
    elif direction == "L":
        pos.x = pos.x - 1
    elif direction == "U":
        pos.y = pos.y + 1
    elif direction == "D":
        pos.y = pos.y - 1
    else:
        raise ValueError("What direction is that? Use 'R', 'L', 'U' or 'D'.")
    return pos        

def next_to(pt1, pt2):
    if pt1 == pt2:
        return True
    elif abs(pt1.x - pt2.x) <= 1 and abs(pt1.y - pt2.y) <= 1:
        return True
    else:
        return False

RIGHTorLEFT = [point(1,0), point(-1,0)]
UPorDOWN = [point(0,1), point(0,-1)]
DIAGs = [point(1,1), point(-1,1), point(-1,-1), point(1,-1)]

def try2follow(possible_moves,lead,follow):
    for move in possible_moves:
        follow_attempt = follow + move
        if next_to(lead, follow_attempt):
            return(follow_attempt)
    raise ValueError("something broke")


def follow(lead, follow):
    if next_to(lead,follow):
        return follow
    elif lead.y == follow.y:
        return try2follow(RIGHTorLEFT,lead,follow)
    elif lead.x == follow.x:
        return try2follow(UPorDOWN,lead,follow)
    else:
        return try2follow(DIAGs,lead,follow)
      

H = point(0,0)
m1 = point(0,0)
m2 = point(0,0)
m3 = point(0,0)
m4 = point(0,0)
m5 = point(0,0)
m6 = point(0,0)
m7 = point(0,0)
m8 = point(0,0)
T = point(0,0)
T_list = []
with open('2022/python/Day9_input.txt') as f:
    for line in f:
        line = line.strip()
        direction = line.split(sep=" ")[0]
        magnitude = int(line.split(sep=" ")[1])
        for step in range(magnitude):
            H = move(direction, H)
            m1 = follow(H,m1)
            m2 = follow(m1,m2)
            m3 = follow(m2,m3)
            m4 = follow(m3,m4)
            m5 = follow(m4,m5)
            m6 = follow(m5,m6)
            m7 = follow(m6,m7)
            m8 = follow(m7,m8)
            T = follow(m8,T)
            if T.name not in T_list:
                T_list.append(str(T))
#print(H, T)
print(len(T_list))