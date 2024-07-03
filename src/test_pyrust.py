"""
File to test rust engine library functionality and execution time
"""


import ChessProject # rust library
import time


gs = ChessProject.GameState()
m = ChessProject.Moves()
p = ChessProject.Perft(max_depth=3)
bmf = ChessProject.BestMoveFinder(search_depth=3)


start = time.time()
# add function below to test execution time

print(f'Execution Time = {time.time() - start}')
