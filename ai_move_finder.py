"""
File contains code for the chess bot
"""


import random
from engine import Move


piece_scores = {'K': 0, 'Q': 9, 'R': 5, 'B': 3, 'N': 3, "P": 1}
CHECKMATE = 1000
STALEMATE = 0


"""
Picks a random move
"""
def findRandomMove(valid_moves) -> Move:
    return valid_moves[random.randint(0, len(valid_moves)-1)] # inclusive bounds


"""
Find best move based on material alone

White wants positive board score, black wants negative board score
"""
def findBestMove(game_state, valid_moves) -> Move:
    turn_multiplier = 1 if game_state.whites_turn else -1
    opps_min_max_score = CHECKMATE
    best_player_move = None
    random.shuffle(valid_moves)
    for player_move in valid_moves:
        game_state.makeMove(player_move)
        opps_moves = game_state.getValidMoves()
        opps_max_score = -CHECKMATE
        for opps_move in opps_moves:
            game_state.makeMove(opps_move)
            if game_state.checkmate:
                score = -turn_multiplier * CHECKMATE
            elif game_state.stalemate:
                score = STALEMATE
            else:
                score = -turn_multiplier * scoreMaterial(game_state.board)
            if score > opps_max_score:
                opps_max_score = score
                # best_player_move = player_move
            game_state.undoMove()
        if opps_max_score < opps_min_max_score:
            opps_min_max_score = opps_max_score
            best_player_move = player_move
        game_state.undoMove()
    return best_player_move


"""
Score the board based on material, white being positive

Can modify to adjust for positional advantage
"""
def scoreMaterial(board) -> int:
    score = 0
    for r in board:
        for sq in r:
            if sq[0] == 'w':
                score += piece_scores[sq[1]]
            elif sq[0] == 'b':
                score -= piece_scores[sq[1]]
    return score