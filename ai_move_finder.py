"""
File contains code for the chess bot
"""


import random
from engine import Move


piece_scores = {'K': 0, 'Q': 9, 'R': 5, 'B': 3, 'N': 3, "P": 1}
CHECKMATE = 1000
STALEMATE = 0
DEPTH = 2


"""
Picks a random move
"""
def findRandomMove(valid_moves) -> Move:
    return valid_moves[random.randint(0, len(valid_moves)-1)] # inclusive bounds


"""
Find best move based on material alone

White wants positive board score, black wants negative board score
Greedy algo with depth 2
"""
def findBestMove(game_state, valid_moves) -> Move:
    turn_multiplier = 1 if game_state.whites_turn else -1
    opps_min_max_score = CHECKMATE
    best_player_move = None
    random.shuffle(valid_moves)
    for player_move in valid_moves:
        game_state.makeMove(player_move)
        opps_moves = game_state.getValidMoves()
        if game_state.checkmate:
            opps_max_score = -CHECKMATE
        elif game_state.stalemate:
            opps_max_score = STALEMATE
        else:
            opps_max_score = -CHECKMATE
            for opps_move in opps_moves:
                game_state.makeMove(opps_move)
                game_state.getValidMoves()
                if game_state.checkmate:
                    score = CHECKMATE
                elif game_state.stalemate:
                    score = STALEMATE
                else:
                    score = -turn_multiplier * scoreMaterial(game_state.board)
                if score > opps_max_score:
                    opps_max_score = score
                game_state.undoMove()

        if opps_max_score < opps_min_max_score:
            opps_min_max_score = opps_max_score
            best_player_move = player_move
        game_state.undoMove()
    return best_player_move


"""
Helper to make first recursive call
"""
def findBestMove(game_state, valid_moves) -> Move:
    global next_move
    next_move = None
    # findMoveMinMax(game_state, valid_moves, DEPTH, game_state.whites_turn)
    findMoveNegaMax(game_state, valid_moves, DEPTH, 1 if game_state.whites_turn else -1)
    return next_move


"""
Recursive min max algo
"""
def findMoveMinMax(game_state, valid_moves, depth, whites_turn) -> int:
    global next_move
    if depth == 0:
        return scoreMaterial(game_state.board)
    
    if whites_turn:
        max_score = -CHECKMATE
        for move in valid_moves:
            game_state.makeMove(move)
            next_moves = game_state.getValidMoves()
            score = findMoveMinMax(game_state, next_moves, depth-1, False)
            if score > max_score:
                max_score = score
                if depth == DEPTH:
                    next_move = move
            game_state.undoMove()
        return max_score

    else:
        min_score = CHECKMATE
        for move in valid_moves:
            game_state.makeMove(move)
            next_moves = game_state.getValidMoves()
            score = findMoveMinMax(game_state, next_moves, depth-1, True)
            if score < min_score:
                min_score = score
                if depth == DEPTH:
                    next_move = move
            game_state.undoMove()
        return min_score


"""
Recursive NegaMax algo
"""
def findMoveNegaMax(game_state, valid_moves, depth, turn_multiplier) -> int:
    global next_move
    if depth == 0:
        return turn_multiplier * scoreBoard(game_state)

    max_score = -CHECKMATE
    for move in valid_moves:
        game_state.makeMove(move)
        next_moves = game_state.getValidMoves()
        score = -findMoveNegaMax(game_state, next_moves, depth-1, -turn_multiplier)
        if score > max_score:
            max_score = score
            if depth == DEPTH:
                next_move = move
        game_state.undoMove()
    return max_score


"""
Positive score is good for white, negative score is good for black
"""
def scoreBoard(game_state) -> int:
    if game_state.checkmate:
        if game_state.whites_turn:
            return -CHECKMATE # black wins
        else:
            return CHECKMATE # white wins
    elif game_state.stalemate:
        return STALEMATE

    score = 0
    for r in game_state.board:
        for sq in r:
            if sq[0] == 'w':
                score += piece_scores[sq[1]]
            elif sq[0] == 'b':
                score -= piece_scores[sq[1]]
    return score


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