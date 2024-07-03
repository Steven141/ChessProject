"""
File to test rust engine library functionality and execution time
"""


import ChessProject # rust library
import time


gs = ChessProject.GameState()
m = ChessProject.Moves()
p = ChessProject.Perft(max_depth=3)
bmf = ChessProject.BestMoveFinder(search_depth=5)


start = time.time()
# add function below to test execution time
bmf.negaMaxAlphaBeta(-1000, 1000, m, gs.wP, gs.wN, gs.wB, gs.wR, gs.wQ, gs.wK, gs.bP, gs.bN, gs.bB, gs.bR, gs.bQ, gs.bK, gs.EP, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, gs.whites_turn, 0)
print(f'Execution Time = {time.time() - start}')
