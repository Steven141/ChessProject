"""
File contains code for the chess bot
"""


import random
import ChessProject # rust engine library


CHECKMATE = 1000
DEPTH = 5


"""
Picks a random move
"""
def findRandomMove(valid_moves) -> str:
    valid_moves_list = [valid_moves[i:i+4] for i in range(0, len(valid_moves), 4)]
    return valid_moves_list[random.randint(0, len(valid_moves_list)-1)] # inclusive bounds


"""
Helper to make first recursive call
"""
def findBestMove(game_state, m, valid_moves) -> str:
    global next_move, move_counter
    next_move = None
    # random.shuffle(valid_moves)
    move_counter = 0
    findMoveNegaMaxAlphaBeta(game_state, m, DEPTH, -CHECKMATE, CHECKMATE, valid_moves)
    print(f'Number of moves: {move_counter}')
    return next_move


"""
Recursive NegaMax algo with alpha beta pruning
"""
def findMoveNegaMaxAlphaBeta(gs, m, depth, alpha, beta, valid_moves) -> int:
    global next_move, move_counter
    bmf = ChessProject.BestMoveFinder(depth)
    max_score = bmf.negaMaxAlphaBeta(alpha, beta, m, gs.wP, gs.wN, gs.wB, gs.wR, gs.wQ, gs.wK, gs.bP, gs.bN, gs.bB, gs.bR, gs.bQ, gs.bK, gs.EP, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, gs.whites_turn, 0)
    move_counter = bmf.move_counter
    # next_move = valid_moves[bmf.best_move_idx:bmf.best_move_idx+4]
    next_move = bmf.next_move
    return max_score
