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
        self.move_functions = {
            'P': self.getPawnMoves,
            'R': self.getRookMoves,
            'N': self.getKnightMoves,
            'B': self.getBishopMoves,
            'Q': self.getQueenMoves,
            'K': self.getKingMoves,
        }
        self.move_log = []
        self.whites_turn = True


    def makeMove(self, move) -> None:
        self.board[move.start_r][move.start_c] = '--'
        self.board[move.end_r][move.end_c] = move.piece_moved
        self.move_log.append(move)
        self.whites_turn = not self.whites_turn


    def undoMove(self) -> None:
        if len(self.move_log) != 0:
            move = self.move_log.pop()
            self.board[move.end_r][move.end_c] = move.piece_captured
            self.board[move.start_r][move.start_c] = move.piece_moved
            self.whites_turn = not self.whites_turn


    """
    Get all moves considering checks
    """
    def getValidMoves(self) -> None:
        return self.getPossibleMoves() # fix later


    """
    Get all moves without concidering checks
    """
    def getPossibleMoves(self) -> list:
        moves = []
        for r in range(len(self.board)):
            for c in range(len(self.board[r])):
                p_turn = self.board[r][c][0]
                if (p_turn == 'w' and self.whites_turn) or (p_turn == 'b' and not self.whites_turn):
                    piece = self.board[r][c][1]
                    self.move_functions[piece](r, c, moves)
        return moves


    def getPawnMoves(self, r, c, moves) -> None:
        if self.whites_turn:
            if self.board[r-1][c] == '--': # one sq advance
                moves.append(Move((r,c), (r-1,c), self.board))
                if r == 6 and self.board[r-2][c] == '--': # two sq advance
                    moves.append(Move((r,c), (r-2, c), self.board))
            if c > 0 and self.board[r-1][c-1][0] == 'b': # left capture
                moves.append(Move((r,c), (r-1, c-1), self.board))
            if c < len(self.board[r])-1 and self.board[r-1][c+1][0] == 'b': # right capture
                moves.append(Move((r,c), (r-1, c+1), self.board))

        else:
            pass


    def getRookMoves(self, r, c, moves) -> None:
        pass


    def getKnightMoves(self, r, c, moves) -> None:
        pass


    def getBishopMoves(self, r, c, moves) -> None:
        pass


    def getQueenMoves(self, r, c, moves) -> None:
        pass


    def getKingMoves(self, r, c, moves) -> None:
        pass


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
        self.move_id = self.start_r*1000 + self.start_c*100 + self.end_r*10 + self.end_c


    def __eq__(self, value: object) -> bool:
        return isinstance(value, Move) and (self.move_id == value.move_id)


    def getChessNotation(self) -> str:
        return f'{self.getRankFile(self.start_r, self.start_c)}{self.getRankFile(self.end_r, self.end_c)}'

    
    def getRankFile(self, r, c) -> str:
        return f'{self.cols_to_files[c]}{self.rows_to_ranks[r]}'
