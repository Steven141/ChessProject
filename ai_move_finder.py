"""
File contains code for the chess bot
"""


import random


def findRandomMove(valid_moves) -> None:
    return valid_moves[random.randint(0, len(valid_moves)-1)] # inclusive bounds