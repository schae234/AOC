#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Wed Jan 11 18:14:46 2023

@author: raegoering

--- Day 11: Monkey in the Middle ---

"""

class Monkey:
    """
    A class to represent a stuff-slinging simian that gets up to shenanigans

    ...

    Attributes
    ----------
    name : str
        Name of monkey, ordered
    items : list
        a list of worry values associated with starting stolen items
    divider : int
        to determine throwing outcome
    operator : string
        evaluated to increase worry values
    P1 : string
        evaluated to determine monkey to throw to if True
    P2 : string
        evaluated to determine monkey to throw to if False
        
    Methods
    -------
    worry():
        Increases the worry value after handeling stolen item
    throw():
        Gives item to different monkey depending on divider test 
    """
    
    
    def __init__(self, name, items, divider, operator, P1, P2):
        """
        Constructs all the necessary attributes for the monkey object.

        Parameters
        ----------
        name : str
            Name of monkey, ordered
        items : list
            a list of worry values associated with starting stolen items
        divider : int
            to determine throwing outcome
        operator : string
            evaluated to increase worry values
        P1 : string
            evaluated to determine monkey to throw to if True
        P2 : string
            evaluated to determine monkey to throw to if False
        """
        
        
        self.name = name
        self.num = int(name.lstrip("Monkey "))
        self.items = items
        self.divider = divider
        self.operator = operator
        self.P1 = P1
        self.P2 = P2
        self.count = 0
        
    def __repr__(self):
        return self.name
    
    def __str__(self):
        return print(
            f'{self.name}:\n'
            f'\tStarting items: {self.items}\n'
            f'\tOperation: new = {self.operator}\n'
            f'\tTest: divisible by {self.divider}\n'
            f'\t\tIf true: throw to monkey {self.P1}\n'
            f'\t\tIf false: throw to monkey {self.P2}\n')
# OG
    # def worry(self):
    #     """
    #     Increases worry value after handeling stolen item using its operator
    #     """

    #     old = self.items.pop(0)
    #     return eval(self.operator)
    
    
    def worry(self, divider_list):
        """
        Increases worry value after handeling stolen item using its operator
        tracks items modulo for each monkey's divider
        """
        if type(self.items[0]) == int:
            old = self.items.pop(0)
            new = eval(self.operator)
            return [new % div for div in divider_list]
        elif type(self.items[0]) == list:
            new = [eval(self.operator) for old in self.items.pop(0)]
            return [new[x] % divider_list[x] for x in range(len(new))]

# OG
    # def throw(self, item, monkey_list):
    #     """
    #     Tosses item to partner monkeys depending on an items division
        
    #     Parameters
    #     ----------
    #     item : int
    #         worry value associated with currently handled stolen item
    #     """

    #     if item % self.divider == 0:
    #         monkey_list[self.P1].items.append(item)
    #     else:
    #         monkey_list[self.P2].items.append(item)
    #     return monkey_list

    def throw(self, item, monkey_list):
        """
        Tosses item to partner monkeys depending on an items division
        
        Parameters
        ----------
        item : int
            worry value associated with currently handled stolen item
        """
        
        if item[self.num] % self.divider == 0:
            monkey_list[self.P1].items.append(item)
        else:
            monkey_list[self.P2].items.append(item)
        return monkey_list

def make_monkeys(file):
    """
    initialize monkeys from text file instructions

    Parameters
    ----------
    file : monkey descriptions

    Returns
    -------
    monkey_dict : dictionary of initialized monkeys

    """
    line_num = 1
    monkey_num = 0
    monkey_dict = {}
    for line in f:
        line = line.strip()
        if line_num % 7 == 1:
            name = line.rstrip(":")
        elif line_num % 7 == 2:
            items = line.split(": ")[1] 
            items = items.split(", ")
            items = [int(item) for item in items]
        elif line_num % 7 == 3:
            operator = line.split(" = ")[1]
        elif line_num % 7 == 4:
            divisor = int(line.split("by ")[1])
        elif(line_num % 7 == 5):
            P1 = int(line.split("monkey ")[1])
        elif(line_num % 7 == 6):
            P2 = int(line.split("monkey ")[1])
            monkey_dict[f"M{str(monkey_num)}"] = Monkey(name, items, divisor, operator, P1, P2)
            monkey_num += 1
        else:
            pass
        line_num += 1
    return monkey_dict

# intialize the example monkeys (before I got smart smart)
# M0 = Monkey("Monkey 0", [79, 98], 23, "old * 19", "M2", "M3")
# M1 = Monkey("Monkey 1", [54, 65, 75, 74], 19, "old + 6", "M2", "M0")
# M2 = Monkey("Monkey 2", [79, 60, 97], 13, "old ** 2", "M1", "M3")
# M3 = Monkey("Monkey 3", [74], 17, "old + 3", "M0", "M1")


with open('2022/python/Day11_input.txt') as f:
    m = make_monkeys(f)
# make list of monkeys        
monkey_round = list(m.values())
monkey_div = [monkey.divider for monkey in monkey_round]

rounds = 0

while rounds < 10000:
    for monkey in monkey_round:
        while monkey.items:
            # print(f"{monkey.name} insepects item with value {monkey.items[0]}")
            item = monkey.worry(monkey_div)
            # print(f"worry level is multiplied to {item}")
            # item = int(item / 3)
            # print(f"worry level is divided to {item}")
            # print(f"{item} divisible by {monkey.divider}?: {item % monkey.divider ==0}")
            # print(f"item thrown to either {monkey.P1} or {monkey.P2}")
            monkey_round = monkey.throw(item, monkey_round)
            monkey.count += 1
            # print(f"{monkey.name} holds {len(monkey.items)} items: {monkey.items}")
    rounds += 1
    if rounds in [1,20] or rounds % 1000 == 0:
        print(f"\n== After round {rounds} ==")
        for monkey in monkey_round:
            print(f"{monkey.name} inspected {monkey.count} items")
    else:
        continue
    # print(f"{rounds} rounds complete.")
    # for monkey in monkey_round:
    #     print(f"{monkey.name} is holding: {monkey.items}")
    
top_monkeys = []
for monkey in monkey_round:
    top_monkeys.append(monkey.count)
    # print(f"{monkey.name} inspected {monkey.count} items")
top_monkeys.sort(reverse = True)
monkey_business = top_monkeys[0] * top_monkeys[1]
# print(top_monkeys)
print(f"\nThe total amount of monkey business is {monkey_business}")
    
