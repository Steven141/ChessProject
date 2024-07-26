"""
File contains code for the chess bot
"""


import random
import time
import ChessProject # rust engine library


CHECKMATE = 10000
DEPTH = 40


"""
Picks a random move
"""
def findRandomMove(valid_moves) -> str:
    valid_moves_list = [valid_moves[i:i+4] for i in range(0, len(valid_moves), 4)]
    return valid_moves_list[random.randint(0, len(valid_moves_list)-1)] # inclusive bounds


"""
Helper to make first recursive call
"""
def findBestMove(game_state, m, z, tt, valid_moves) -> str:
    global next_move, move_counter
    next_move = None
    move_counter = 0
    start_t = time.time()
    findMoveNegaMaxAlphaBeta(game_state, m, z, tt, DEPTH, -CHECKMATE, CHECKMATE, valid_moves)
    return next_move


"""
Recursive NegaMax algo with alpha beta pruning
"""
def findMoveNegaMaxAlphaBeta(gs, m, z, tt, depth, alpha, beta, valid_moves) -> None:
    global next_move, move_counter
    bmf = ChessProject.BestMoveFinder(depth)
    bmf.searchPosition(m, z, tt, gs.bitboards, gs.castle_rights, gs.hash_key, gs.whites_turn)
    move_counter = bmf.move_counter
    next_move = bmf.pv_table[0][0]
