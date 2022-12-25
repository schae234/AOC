#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Sun Dec 25 12:35:49 2022

@author: raegoering

"""

def get_outcome(c1, c2):
    """
    Parameters
    ----------
    c1 : str
    c2 : str

    Returns str
    -------
    This function evaluates the outcome of a single game of RPS.

    """
    c1code = {"A": "Rock", "B": "Paper", "C": "Scissors"}
    c2code = {"X": "Rock", "Y": "Paper", "Z": "Scissors"}
    losers = [("Rock", "Scissors"), ("Paper", "Rock"), ("Scissors", "Paper")]
    hand1 = c1code[c1]
    if c2 in ["X", "Y", "Z"]:
        hand2 = c2code[c2]
    else:
        hand2 = c2
    if hand1 == hand2:
        outcome = "draw"
    elif (hand1, hand2) in losers:
        outcome = "lose"
    else:
        outcome = "win"
    return outcome


def get_score(c2, outcome):
    """
    Parameters
    ----------
    c2 : str
    outcome : str

    Returns int
    -------
    This function evaluates the score for a single game of RPS.

    """
    c2code = {"X": "Rock", "Y": "Paper", "Z": "Scissors"}
    RPS_values = {"Rock": 1, "Paper": 2, "Scissors": 3}
    outcome_values = {"lose": 0, "draw": 3, "win": 6}
    if c2 in ["X", "Y", "Z"]:
        hand2 = c2code[c2]
    else:
        hand2 = c2
    score = RPS_values[hand2] + outcome_values[outcome]
    return score


def choose_hand(c1, c2):
    """
    Parameters
    ----------
    c1 : str
    c2 : srr

    Returns str
    -------
    This function determines which hand to play to meet the desired outcome

    """
    c1code = {"A": "Rock", "B": "Paper", "C": "Scissors"}
    new_c2code = {"X": "lose", "Y": "draw", "Z": "win"}
    losers = {"Rock": "Scissors", "Paper": "Rock", "Scissors": "Paper"}
    winners = {"Rock": "Paper", "Paper": "Scissors", "Scissors": "Rock"}
    hand1 = c1code[c1]
    if new_c2code[c2] == "draw":
        hand2 = hand1
    elif new_c2code[c2] == "lose":
        hand2 = losers[hand1]
    else:
        hand2 = winners[hand1]
    return hand2


"""
--- Day 2: Rock Paper Scissors ---

"""

score = 0

with open('Documents/AOC/2022/python/Day2_input.txt') as f:
    for line in f:
        columns = line.split()
        outcome = get_outcome(columns[0], columns[1])
        score += get_score(columns[1], outcome)

print(f"Following your first idea, you score {score}")

score = 0

with open('Documents/AOC/2022/python/Day2_input.txt') as f:
    for line in f:
        columns = line.split()
        hand = choose_hand(columns[0], columns[1])
        outcome = get_outcome(columns[0], hand)
        score += get_score(hand, outcome)

print(f"With the your new understanding, you score {score}")
