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
        self.wk_location = (7, 4)
        self.bk_location = (0, 4)
        self.checkmate = False
        self.stalemate = False
        self.enpassant_possible = () # cord where enpassant possible


    def makeMove(self, move) -> None:
        self.board[move.start_r][move.start_c] = '--'
        self.board[move.end_r][move.end_c] = move.piece_moved
        self.move_log.append(move)
        self.whites_turn = not self.whites_turn

        if move.piece_moved == 'wK':
            self.wk_location = (move.end_r, move.end_c)
        elif move.piece_moved == 'bK':
            self.bk_location = (move.end_r, move.end_c)

        if move.is_pawn_promo:
            self.board[move.end_r][move.end_c] = f'{move.piece_moved[0]}Q'

        if move.is_enpassant_move: # capture pawn
            self.board[move.start_r][move.end_c] = '--'

        if move.piece_moved[1] == 'P' and abs(move.end_r - move.start_r) == 2: # two sq pawn advances
            self.enpassant_possible = ((move.start_r + move.end_r) // 2, move.end_c)
        else:
            self.enpassant_possible = ()


    def undoMove(self) -> None:
        if len(self.move_log) != 0:
            move = self.move_log.pop()
            self.board[move.end_r][move.end_c] = move.piece_captured
            self.board[move.start_r][move.start_c] = move.piece_moved
            self.whites_turn = not self.whites_turn

            if move.piece_moved == 'wK':
                self.wk_location = (move.start_r, move.start_c)
            elif move.piece_moved == 'bK':
                self.bk_location = (move.start_r, move.start_c)

            # TODO: does not work for chained undos
            if move.is_enpassant_move: # undo enpassant
                self.board[move.end_r][move.end_c] = '--'
                self.board[move.start_r][move.end_c] = move.piece_captured
                self.enpassant_possible = (move.end_r, move.end_c)
            if move.piece_moved[1] == 'P' and abs(move.end_r - move.start_r) == 2: # undo two sq pawn advances
                self.enpassant_possible = ()


    """
    Get all moves considering checks
    """
    def getValidMoves(self) -> None:
        # TODO: look to upgrade to better algo for checks
        temp_enpassant_possible = self.enpassant_possible
        # generate all possible moves
        moves = self.getPossibleMoves()
        # make each move
        for i in range(len(moves)-1, -1, -1):
            self.makeMove(moves[i]) # switches turns
            # generate all opps moves and check if attack king
            self.whites_turn = not self.whites_turn
            # if they attack king, move was not valid
            if self.inCheck():
                moves.remove(moves[i])
            self.whites_turn = not self.whites_turn
            self.undoMove() # switches turns

        if len(moves) == 0:
            if self.inCheck():
                self.checkmate = True
            else:
                self.stalemate = True
        else:
            # for undo cases
            self.checkmate = False
            self.stalemate = False

        self.enpassant_possible = temp_enpassant_possible
        return moves


    """
    Determine if current players is in check
    """
    def inCheck(self) -> bool:
        if self.whites_turn:
            return self.squareUnderAttack(self.wk_location[0], self.wk_location[1])
        else:
            return self.squareUnderAttack(self.bk_location[0], self.bk_location[1])


    """
    Determine if opp can attack square (r, c)
    """
    def squareUnderAttack(self, r, c) -> bool:
        self.whites_turn = not self.whites_turn # switch to opps turn
        opp_moves = self.getPossibleMoves()
        self.whites_turn = not self.whites_turn
        for move in opp_moves:
            if move.end_r == r and move.end_c == c:
                return True
        return False


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
        # TODO: try combining if-else later
        if self.whites_turn:
            if self.board[r-1][c] == '--': # one sq advance
                moves.append(Move((r,c), (r-1,c), self.board))
                if r == 6 and self.board[r-2][c] == '--': # two sq advance
                    moves.append(Move((r,c), (r-2, c), self.board))
            if c > 0:
                if self.board[r-1][c-1][0] == 'b': # left capture
                    moves.append(Move((r,c), (r-1, c-1), self.board))
                elif (r-1, c-1) == self.enpassant_possible:
                    moves.append(Move((r,c), (r-1, c-1), self.board, is_enpassant_move=True))
            if c < len(self.board[r]) - 1:
                if self.board[r-1][c+1][0] == 'b': # right capture
                    moves.append(Move((r,c), (r-1, c+1), self.board))
                elif (r-1, c+1) == self.enpassant_possible:
                    moves.append(Move((r,c), (r-1, c+1), self.board, is_enpassant_move=True))

        else:
            if self.board[r+1][c] == '--': # one sq advance
                moves.append(Move((r,c), (r+1,c), self.board))
                if r == 1 and self.board[r+2][c] == '--': # two sq advance
                    moves.append(Move((r,c), (r+2, c), self.board))
            if c > 0:
                if self.board[r+1][c-1][0] == 'w': # left capture
                    moves.append(Move((r,c), (r+1, c-1), self.board))
                elif (r+1, c-1) == self.enpassant_possible:
                    moves.append(Move((r,c), (r+1, c-1), self.board, is_enpassant_move=True))
            if c < len(self.board[r]) - 1:
                if self.board[r+1][c+1][0] == 'w': # right capture
                    moves.append(Move((r,c), (r+1, c+1), self.board))
                elif (r+1, c+1) == self.enpassant_possible:
                    moves.append(Move((r,c), (r+1, c+1), self.board, is_enpassant_move=True))


    def getRookMoves(self, r, c, moves) -> None:
        enemy_color = 'b' if self.whites_turn else 'w'
        dirs = ((-1, 0), (1, 0), (0, 1), (0, -1))
        for d in dirs:
            for i in range(1, len(self.board)):
                end_r = r + d[0] * i
                end_c = c + d[1] * i
                if 0 <= end_r < len(self.board) and 0 <= end_c < len(self.board[r]):
                    end_piece = self.board[end_r][end_c]
                    if end_piece == '--':
                        moves.append(Move((r,c), (end_r, end_c), self.board))
                    elif end_piece[0] == enemy_color:
                        moves.append(Move((r,c), (end_r, end_c), self.board))
                        break
                    else:
                        break
                else:
                    break


    def getKnightMoves(self, r, c, moves) -> None:
        enemy_color = 'b' if self.whites_turn else 'w'
        dirs = ((-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1))
        for d in dirs:
            end_r = r + d[0]
            end_c = c + d[1]
            if 0 <= end_r < len(self.board) and 0 <= end_c < len(self.board[r]):
                end_piece = self.board[end_r][end_c]
                if end_piece == '--' or end_piece[0] == enemy_color:
                    moves.append(Move((r,c), (end_r, end_c), self.board))


    def getBishopMoves(self, r, c, moves) -> None:
        # TODO: try combining with rook moves
        enemy_color = 'b' if self.whites_turn else 'w'
        dirs = ((-1, -1), (1, 1), (-1, 1), (1, -1))
        for d in dirs:
            for i in range(1, len(self.board)):
                end_r = r + d[0] * i
                end_c = c + d[1] * i
                if 0 <= end_r < len(self.board) and 0 <= end_c < len(self.board[r]):
                    end_piece = self.board[end_r][end_c]
                    if end_piece == '--':
                        moves.append(Move((r,c), (end_r, end_c), self.board))
                    elif end_piece[0] == enemy_color:
                        moves.append(Move((r,c), (end_r, end_c), self.board))
                        break
                    else:
                        break
                else:
                    break


    def getQueenMoves(self, r, c, moves) -> None:
        self.getRookMoves(r, c, moves)
        self.getBishopMoves(r, c, moves)


    def getKingMoves(self, r, c, moves) -> None:
        enemy_color = 'b' if self.whites_turn else 'w'
        dirs = ((-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1))
        for i in range(len(dirs)):
            end_r = r + dirs[i][0]
            end_c = c + dirs[i][1]
            if 0 <= end_r < len(self.board) and 0 <= end_c < len(self.board[r]):
                end_piece = self.board[end_r][end_c]
                if end_piece == '--' or end_piece[0] == enemy_color:
                    moves.append(Move((r,c), (end_r, end_c), self.board))


class Move():
    # the following converts between cordinate and rank notation
    rank_to_rows = {'1': 7, '2': 6, '3': 5, '4': 4,
                    '5': 3, '6': 2, '7': 1, '8': 0}
    rows_to_ranks = {v: k for k, v in rank_to_rows.items()}
    files_to_cols = {'a': 0, 'b': 1, 'c': 2, 'd': 3,
                     'e': 4, 'f': 5, 'g': 6, 'h': 7}
    cols_to_files = {v: k for k, v in files_to_cols.items()}


    def __init__(self, start_sq, end_sq, board, is_enpassant_move=False) -> None:
        self.start_r, self.start_c = start_sq[0], start_sq[1]
        self.end_r, self.end_c = end_sq[0], end_sq[1]
        self.piece_moved = board[self.start_r][self.start_c]
        self.piece_captured = board[self.end_r][self.end_c]
        self.is_pawn_promo = (self.piece_moved == 'wP' and self.end_r == 0) or (self.piece_moved == 'bP' and self.end_r == 7)
        self.is_enpassant_move = is_enpassant_move
        if self.is_enpassant_move:
            self.piece_captured = 'wP' if self.piece_moved == 'bP' else 'bP'

        self.move_id = self.start_r*1000 + self.start_c*100 + self.end_r*10 + self.end_c


    def __eq__(self, value: object) -> bool:
        return isinstance(value, Move) and (self.move_id == value.move_id)


    def getChessNotation(self) -> str:
        return f'{self.getRankFile(self.start_r, self.start_c)}{self.getRankFile(self.end_r, self.end_c)}'

    
    def getRankFile(self, r, c) -> str:
        return f'{self.cols_to_files[c]}{self.rows_to_ranks[r]}'
