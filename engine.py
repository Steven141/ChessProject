"""
File contains data stored from chess game.

Keeps track of possible moves and old moves.
"""

class GameState():
    def __init__(self) -> None:
        """
        In this 8x8 game board, the first char represents color
        and the second char represents type of piece. Empty 
        square is denoted with '--'.
        """
        self.board = [
            ['bR', 'bN', 'bB', 'bQ', 'bK', 'bB', 'bN', 'bR'],
            ['bP', 'bP', 'bP', 'bP', 'bP', 'bP', 'bP', 'bP'],
            ['--', '--', '--', '--', '--', '--', '--', '--'],
            ['--', '--', '--', '--', '--', '--', '--', '--'],
            ['--', '--', '--', '--', '--', '--', '--', '--'],
            ['--', '--', '--', '--', '--', '--', '--', '--'],
            ['wP', 'wP', 'wP', 'wP', 'wP', 'wP', 'wP', 'wP'],
            ['wR', 'wN', 'wB', 'wQ', 'wK', 'wB', 'wN', 'wR'],
        ]
        self.move_log = []
        self.whites_turn = True


    def makeMove(self, move) -> None:
        self.board[move.start_r][move.start_c] = '--'
        self.board[move.end_r][move.end_c] = move.piece_moved
        self.move_log.append(move)
        self.whites_turn = not self.whites_turn


class Move():
    # the following converts between cordinate and rank notation
    rank_to_rows = {'1': 7, '2': 6, '3': 5, '4': 4,
                    '5': 3, '6': 2, '7': 1, '8': 0}
    rows_to_ranks = {v: k for k, v in rank_to_rows.items()}
    files_to_cols = {'a': 0, 'b': 1, 'c': 2, 'd': 3,
                     'e': 4, 'f': 5, 'g': 6, 'h': 7}
    cols_to_files = {v: k for k, v in files_to_cols.items()}


    def __init__(self, start_sq, end_sq, board) -> None:
        self.start_r, self.start_c = start_sq[0], start_sq[1]
        self.end_r, self.end_c = end_sq[0], end_sq[1]
        self.piece_moved = board[self.start_r][self.start_c]
        self.piece_captured = board[self.end_r][self.end_c]


    def getChessNotation(self) -> str:
        return f'{self.getRankFile(self.start_r, self.start_c)}{self.getRankFile(self.end_r, self.end_c)}'

    
    def getRankFile(self, r, c) -> str:
        return f'{self.cols_to_files[c]}{self.rows_to_ranks[r]}'
