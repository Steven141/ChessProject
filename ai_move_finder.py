"""
File contains code for the chess bot
"""


import random
from engine import Move


piece_scores = {'K': 0, 'Q': 9, 'R': 5, 'B': 3, 'N': 3, "P": 1}
knight_scores = [
    [1, 1, 1, 1, 1, 1, 1, 1],
    [1, 2, 2, 2, 2, 2, 2, 1],
    [1, 2, 3, 3, 3, 3, 2, 1],
    [1, 2, 3, 4, 4, 3, 2, 1],
    [1, 2, 3, 4, 4, 3, 2, 1],
    [1, 2, 3, 3, 3, 3, 2, 1],
    [1, 2, 2, 2, 2, 2, 2, 1],
    [1, 1, 1, 1, 1, 1, 1, 1],
]
bishop_scores = [
    [4, 3, 2, 1, 1, 2, 3, 4],
    [3, 4, 3, 2, 2, 3, 4, 3],
    [2, 3, 4, 3, 3, 4, 3, 2],
    [1, 2, 3, 4, 4, 3, 2, 1],
    [1, 2, 3, 4, 4, 3, 2, 1],
    [2, 3, 4, 3, 3, 4, 3, 2],
    [3, 4, 3, 2, 2, 3, 4, 3],
    [4, 3, 2, 1, 1, 2, 3, 4],
]
queen_scores = [
    [1, 1, 1, 3, 1, 1, 1, 1],
    [1, 2, 3, 3, 3, 1, 1, 1],
    [1, 4, 3, 3, 3, 4, 2, 1],
    [1, 2, 3, 3, 3, 2, 2, 1],
    [1, 2, 3, 3, 3, 2, 2, 1],
    [1, 4, 3, 3, 3, 4, 2, 1],
    [1, 2, 3, 3, 3, 1, 1, 1],
    [1, 1, 1, 3, 1, 1, 1, 1],
]
rook_scores = [
    [4, 3, 4, 4, 4, 4, 3, 4],
    [4, 4, 4, 4, 4, 4, 4, 4],
    [1, 1, 2, 3, 3, 2, 1, 1],
    [1, 2, 3, 4, 4, 3, 2, 1],
    [1, 2, 3, 4, 4, 3, 2, 1],
    [1, 1, 2, 3, 3, 2, 1, 1],
    [4, 4, 4, 4, 4, 4, 4, 4],
    [4, 3, 4, 4, 4, 4, 3, 4],
]
w_pawn_scores = [
    [8, 8, 8, 8, 8, 8, 8, 8],
    [8, 8, 8, 8, 8, 8, 8, 8],
    [5, 6, 6, 7, 7, 6, 6, 5],
    [2, 3, 3, 5, 5, 3, 3, 2],
    [1, 2, 3, 4, 4, 3, 2, 1],
    [1, 1, 2, 3, 3, 2, 1, 1],
    [1, 1, 1, 0, 0, 1, 1, 1],
    [0, 0, 0, 0, 0, 0, 0, 0],
]
b_pawn_scores = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [1, 1, 1, 0, 0, 1, 1, 1],
    [1, 1, 2, 3, 3, 2, 1, 1],
    [1, 2, 3, 4, 4, 3, 2, 1],
    [2, 3, 3, 5, 5, 3, 3, 2],
    [5, 6, 6, 7, 7, 6, 6, 5],
    [8, 8, 8, 8, 8, 8, 8, 8],
    [8, 8, 8, 8, 8, 8, 8, 8],
]
piece_position_scores = {
    'N': knight_scores,
    'Q': queen_scores,
    'B': bishop_scores,
    'R': rook_scores,
    'wP': w_pawn_scores,
    'bP': b_pawn_scores,
}
CHECKMATE = 1000
STALEMATE = 0
DEPTH = 2


"""
Picks a random move
"""
def findRandomMove(valid_moves) -> Move:
    return valid_moves[random.randint(0, len(valid_moves)-1)] # inclusive bounds


"""
Find best move based on minmax algo without recursion

White wants positive board score, black wants negative board score
Greedy algo with depth 2
"""
def findBestMoveMinMaxNoRecursion(game_state, valid_moves) -> Move:
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
def findBestMove(game_state, valid_moves, return_q) -> Move:
    global next_move, move_counter
    next_move = None
    random.shuffle(valid_moves)
    move_counter = 0
    # findMoveMinMax(game_state, valid_moves, DEPTH, game_state.whites_turn)
    # findMoveNegaMax(game_state, valid_moves, DEPTH, 1 if game_state.whites_turn else -1)
    findMoveNegaMaxAlphaBeta(game_state, valid_moves, DEPTH, -CHECKMATE, CHECKMATE, 1 if game_state.whites_turn else -1)
    print(f'Number of moves: {move_counter}')
    return_q.put(next_move)


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
    global next_move, move_counter
    move_counter += 1
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
Recursive NegaMax algo with alpha beta pruning
"""
def findMoveNegaMaxAlphaBeta(game_state, valid_moves, depth, alpha, beta, turn_multiplier) -> int:
    global next_move, move_counter
    move_counter += 1
    if depth == 0:
        return turn_multiplier * scoreBoard(game_state)

    max_score = -CHECKMATE
    for move in valid_moves:
        game_state.makeMove(move)
        next_moves = game_state.getValidMoves()
        score = -findMoveNegaMaxAlphaBeta(game_state, next_moves, depth-1, -beta, -alpha, -turn_multiplier)
        if score > max_score:
            max_score = score
            if depth == DEPTH:
                next_move = move
                print(f'Considering {move} with score: {score}')
        game_state.undoMove()

        if max_score > alpha:
            alpha = max_score
        if alpha >= beta:
            break
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
    for r in range(len(game_state.board)):
        for c in range(len(game_state.board[r])):
            sq = game_state.board[r][c]
            if sq != '--':
                # score positionally
                piece_position_score = 0
                if sq[1] != 'K': # no position table for king
                    if sq[1] == 'P':
                        piece_position_score = piece_position_scores[sq][r][c]
                    else:
                        piece_position_score = piece_position_scores[sq[1]][r][c]

                if sq[0] == 'w':
                    score += piece_scores[sq[1]] + piece_position_score * 0.1
                elif sq[0] == 'b':
                    score -= (piece_scores[sq[1]] + piece_position_score * 0.1)
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