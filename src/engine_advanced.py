"""
File contains data stored from chess game.

Keeps track of possible moves and old moves.
"""


import copy


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

        # advanced valid move algo
        self.is_in_check = False
        self.pins = []
        self.checks = []

        self.checkmate = False
        self.stalemate = False
        self.enpassant_possible = () # cord where enpassant possible
        self.enpassant_possible_log = [self.enpassant_possible]
        self.current_castling_rights = CastleRights(True, True, True, True)
        self.castle_rights_log = [copy.deepcopy(self.current_castling_rights)]


    def makeMove(self, move) -> None:
        self.board[move.start_r][move.start_c] = '--'
        self.board[move.end_r][move.end_c] = move.piece_moved
        self.move_log.append(move)
        self.whites_turn = not self.whites_turn

        # update kings location
        if move.piece_moved == 'wK':
            self.wk_location = (move.end_r, move.end_c)
        elif move.piece_moved == 'bK':
            self.bk_location = (move.end_r, move.end_c)

        # pawn promotion move
        if move.is_pawn_promo:
            # promoted_piece = input('Promote to Q, R, B, or N: ') # TODO
            promoted_piece = 'Q'
            self.board[move.end_r][move.end_c] = f'{move.piece_moved[0]}{promoted_piece}'

        # enpassant move
        if move.is_enpassant_move:
            self.board[move.start_r][move.end_c] = '--'

        # update enpassant_possible variable
        if move.piece_moved[1] == 'P' and abs(move.end_r - move.start_r) == 2: # two sq pawn advances
            self.enpassant_possible = ((move.start_r + move.end_r) // 2, move.end_c)
        else:
            self.enpassant_possible = ()
        self.enpassant_possible_log.append(self.enpassant_possible)

        # castling move
        if move.is_castle_move:
            if move.end_c - move.start_c == 2: # king side castle
                self.board[move.end_r][move.end_c-1] = self.board[move.end_r][move.end_c+1]
                self.board[move.end_r][move.end_c+1] = '--'
            else: # queen side castle
                self.board[move.end_r][move.end_c+1] = self.board[move.end_r][move.end_c-2]
                self.board[move.end_r][move.end_c-2] = '--'

        # update castling rights ie. rook or king move
        self.updateCastleRights(move)
        self.castle_rights_log.append(copy.deepcopy(self.current_castling_rights))


    def undoMove(self) -> None:
        if len(self.move_log) != 0:
            move = self.move_log.pop()
            self.board[move.end_r][move.end_c] = move.piece_captured
            self.board[move.start_r][move.start_c] = move.piece_moved
            self.whites_turn = not self.whites_turn

            # update kings position for checks
            if move.piece_moved == 'wK':
                self.wk_location = (move.start_r, move.start_c)
            elif move.piece_moved == 'bK':
                self.bk_location = (move.start_r, move.start_c)

            # undo enpassant - sus end part 8
            if move.is_enpassant_move:
                self.board[move.end_r][move.end_c] = '--'
                self.board[move.start_r][move.end_c] = move.piece_captured
            self.enpassant_possible_log.pop()
            self.enpassant_possible = self.enpassant_possible_log[-1]

            # undo castling rights
            self.castle_rights_log.pop()
            self.current_castling_rights = copy.deepcopy(self.castle_rights_log[-1])

            # undo castling move
            if move.is_castle_move:
                if move.end_c - move.start_c == 2: # king side castle
                    self.board[move.end_r][move.end_c+1] = self.board[move.end_r][move.end_c-1]
                    self.board[move.end_r][move.end_c-1] = '--'
                else: # queen side castle
                    self.board[move.end_r][move.end_c-2] = self.board[move.end_r][move.end_c+1]
                    self.board[move.end_r][move.end_c+1] = '--'

            self.checkmate = False
            self.stalemate = False



    """
    Update castling rights given a move
    """
    def updateCastleRights(self, move) -> None:
        if move.piece_moved == 'wK':
            self.current_castling_rights.wqs = False
            self.current_castling_rights.wks = False
        elif move.piece_moved == 'bK':
            self.current_castling_rights.bqs = False
            self.current_castling_rights.bks = False
        elif move.piece_moved == 'wR':
            if move.start_r == 7:
                if move.start_c == 0:
                    self.current_castling_rights.wqs = False
                elif move.start_c == 7:
                    self.current_castling_rights.wks = False
        elif move.piece_moved == 'bR':
            if move.start_r == 0:
                if move.start_c == 0:
                    self.current_castling_rights.bqs = False
                elif move.start_c == 7:
                    self.current_castling_rights.bks = False

        # rook is captured
        if move.piece_captured == 'wR':
            if move.end_r == 7:
                if move.end_c == 0:
                    self.current_castling_rights.wqs = False
                elif move.end_c == 7:
                    self.current_castling_rights.wks = False
        elif move.piece_captured == 'bR':
            if move.end_r == 0:
                if move.end_c == 0:
                    self.current_castling_rights.bqs = False
                elif move.end_c == 7:
                    self.current_castling_rights.bks = False


    """
    Get all moves considering checks
    """
    def getValidMoves(self) -> list:
        moves = []
        self.is_in_check, self.pins, self.checks = self.checkForPinsAndChecks()
        if self.whites_turn:
            king_r = self.wk_location[0]
            king_c = self.wk_location[1]
        else:
            king_r = self.bk_location[0]
            king_c = self.bk_location[1]
        
        if self.is_in_check:
            if len(self.checks) == 1: # only 1 check, block check or move king
                moves = self.getPossibleMoves()
                # to block a check you must move a piece into one of the squares between the enemy piece and king
                check = self.checks[0] # check info
                check_r = check[0]
                check_c = check[1]
                piece_checking = self.board[check_r][check_c]
                valid_squares = [] # squares that pieces can move to

                # if knight, must capture knight or move king, other pieces can be blocked
                if piece_checking[1] == 'N':
                    valid_squares = [(check_r, check_c)]
                else:
                    for i in range(1, 8):
                        valid_square = (king_r + check[2] * i, king_c + check[3] * i)
                        valid_squares.append(valid_square)
                        if valid_square[0] == check_r and valid_square[1] == check_c:
                            break

                # get rid of any moves that don't block check or move king
                for i in range(len(moves)-1, -1, -1):
                    if moves[i].piece_moved[1] != 'K': # move does not move king so must block or capture
                        if not (moves[i].end_r, moves[i].end_c) in valid_squares:
                            moves.remove(moves[i])

            else: # double check, king has to move
                self.getKingMoves(king_r, king_c, moves)

        else: # no check, all moves valid
            moves = self.getPossibleMoves()
            if self.whites_turn:
                self.getCastleMoves(self.wk_location[0], self.wk_location[1], moves)
            else:
                self.getCastleMoves(self.bk_location[0], self.bk_location[1], moves)

        if len(moves) == 0:
            if self.is_in_check:
                self.checkmate = True
            else:
                self.stalemate = True
        else:
            self.checkmate = False
            self.stalemate = False

        return moves
    

    """
    Returns if the player is in check, a list of pins, and a list of checks
    """
    def checkForPinsAndChecks(self) -> tuple[bool, list[tuple[int, int, int, int]], list[tuple[int, int, int, int]]]:
        pins = [] # squares where the alligned pinned piece is and direction pinned from
        checks = [] # squares where enemy is applying a check
        is_in_check = False
        if self.whites_turn:
            enemy_color = 'b'
            ally_color = 'w'
            start_r = self.wk_location[0]
            start_c = self.wk_location[1]
        else:
            enemy_color = 'w'
            ally_color = 'b'
            start_r = self.bk_location[0]
            start_c = self.bk_location[1]

        # check outward from king for pins and checks, keep track of pins
        dirs = ((-1, 0), (0, -1), (1, 0), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1))
        for j in range(len(dirs)):
            d = dirs[j]
            possible_pin = () # reset possible pins
            for i in range(1, 8):
                end_r = start_r + d[0] * i
                end_c = start_c + d[1] * i
                if 0 <= end_r < 8 and 0 <= end_c < 8:
                    end_piece = self.board[end_r][end_c]
                    if end_piece[0] == ally_color and end_piece[1] != 'K': # phantom king from fake moving king in getKingMoves()
                        if possible_pin == (): # 1st allied piece could be pinned
                            possible_pin = (end_r, end_c, d[0], d[1])
                        else: # 2nd allied piece, so no pin or check possible in direction
                            break
                    elif end_piece[0] == enemy_color:
                        piece_type = end_piece[1]
                        # 5 possibilities causing check
                        # 1. orthogonally away from king and piece is rook
                        rook_cond = 0 <= j <= 3 and piece_type == 'R'
                        # 2. diagonally away from king and piece is bishop
                        bishop_cond = 4 <= j <= 7 and piece_type == 'B'
                        # 3. 1 square away diagonally from king and piece is a pawn
                        pawn_cond = i == 1 and piece_type == 'P' and ((enemy_color == 'w' and 6 <= j <= 7) or (enemy_color == 'b' and 4 <= j <= 5))
                        # 4. any direction and piece is a queen
                        queen_cond = piece_type == 'Q'
                        # 5. any direction 1 square away and piece is a king (prevent king move to square controlled by another king)
                        king_cond = i == 1 and piece_type == 'K'

                        if rook_cond or bishop_cond or pawn_cond or queen_cond or king_cond:
                            if possible_pin == (): # no piece blocking
                                is_in_check = True
                                checks.append((end_r, end_c, d[0], d[1]))
                                break
                            else: # piece blocking
                                pins.append(possible_pin)
                                break
                        else: # enemy piece not applying check
                            break
                else: # off board
                    break

        # look for knight checks
        knight_moves = ((-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1))
        for m in knight_moves:
            end_r = start_r + m[0]
            end_c = start_c + m[1]
            if 0 <= end_r < 8 and 0 <= end_c < 8:
                end_piece = self.board[end_r][end_c]
                if end_piece[0] == enemy_color and end_piece[1] == 'N':
                    is_in_check = True
                    checks.append((end_r, end_c, d[0], d[1]))

        return is_in_check, pins, checks


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
        piece_pinned = False
        pin_direction = ()
        for i in range(len(self.pins)-1, -1, -1):
            if self.pins[i][0] == r and self.pins[i][1] == c:
                piece_pinned = True
                pin_direction = (self.pins[i][2], self.pins[i][3])
                self.pins.remove(self.pins[i])
                break

        if self.whites_turn:
            king_r, king_c = self.wk_location
            if self.board[r-1][c] == '--': # one sq advance
                if not piece_pinned or pin_direction == (-1, 0):
                    moves.append(Move((r,c), (r-1,c), self.board))
                    if r == 6 and self.board[r-2][c] == '--': # two sq advance
                        moves.append(Move((r,c), (r-2, c), self.board))
            if c > 0:
                if not piece_pinned or pin_direction == (-1, -1):
                    if self.board[r-1][c-1][0] == 'b': # left capture
                        moves.append(Move((r,c), (r-1, c-1), self.board))
                    elif (r-1, c-1) == self.enpassant_possible: # left enpassent
                        is_attacking_piece = is_blocking_piece = False
                        if king_r == r:
                            if king_c < c: # king is left of pawn
                                # inside between king and pawn, outside range between pawn and border
                                inside_range = range(king_c+1, c-1)
                                outside_range = range(c+1, 8)
                            else: # king is right of pawn
                                inside_range = range(king_c-1, c, -1)
                                outside_range = range(c-2, -1, -1)
                            for i in inside_range:
                                if self.board[r][i] != '--':
                                    is_blocking_piece = True
                            for i in outside_range:
                                sq = self.board[r][i]
                                if sq[0] == 'b' and (sq[1] == 'R' or sq[1] == 'Q'): # attacking piece
                                    is_attacking_piece = True
                                elif sq != '--':
                                    is_blocking_piece = True
                        if not is_attacking_piece or is_blocking_piece:
                            moves.append(Move((r,c), (r-1, c-1), self.board, is_enpassant_move=True))
            if c < len(self.board[r]) - 1:
                if not piece_pinned or pin_direction == (-1, 1):
                    if self.board[r-1][c+1][0] == 'b': # right capture
                        moves.append(Move((r,c), (r-1, c+1), self.board))
                    elif (r-1, c+1) == self.enpassant_possible: # right enpassent
                        is_attacking_piece = is_blocking_piece = False
                        if king_r == r:
                            if king_c < c: # king is left of pawn
                                # inside between king and pawn, outside range between pawn and border
                                inside_range = range(king_c+1, c)
                                outside_range = range(c+2, 8)
                            else: # king is right of pawn
                                inside_range = range(king_c-1, c+1, -1)
                                outside_range = range(c-1, -1, -1)
                            for i in inside_range:
                                if self.board[r][i] != '--':
                                    is_blocking_piece = True
                            for i in outside_range:
                                sq = self.board[r][i]
                                if sq[0] == 'b' and (sq[1] == 'R' or sq[1] == 'Q'): # attacking piece
                                    is_attacking_piece = True
                                elif sq != '--':
                                    is_blocking_piece = True
                        if not is_attacking_piece or is_blocking_piece:
                            moves.append(Move((r,c), (r-1, c+1), self.board, is_enpassant_move=True))

        else:
            king_r, king_c = self.bk_location
            if self.board[r+1][c] == '--': # one sq advance
                if not piece_pinned or pin_direction == (1, 0):
                    moves.append(Move((r,c), (r+1,c), self.board))
                    if r == 1 and self.board[r+2][c] == '--': # two sq advance
                        moves.append(Move((r,c), (r+2, c), self.board))
            if c > 0:
                if not piece_pinned or pin_direction == (1, -1):
                    if self.board[r+1][c-1][0] == 'w': # left capture
                        moves.append(Move((r,c), (r+1, c-1), self.board))
                    elif (r+1, c-1) == self.enpassant_possible: # left enpassent
                        is_attacking_piece = is_blocking_piece = False
                        if king_r == r:
                            if king_c < c: # king is left of pawn
                                # inside between king and pawn, outside range between pawn and border
                                inside_range = range(king_c+1, c-1)
                                outside_range = range(c+1, 8)
                            else: # king is right of pawn
                                inside_range = range(king_c-1, c, -1)
                                outside_range = range(c-2, -1, -1)
                            for i in inside_range:
                                if self.board[r][i] != '--':
                                    is_blocking_piece = True
                            for i in outside_range:
                                sq = self.board[r][i]
                                if sq[0] == 'w' and (sq[1] == 'R' or sq[1] == 'Q'): # attacking piece
                                    is_attacking_piece = True
                                elif sq != '--':
                                    is_blocking_piece = True
                        if not is_attacking_piece or is_blocking_piece:
                            moves.append(Move((r,c), (r+1, c-1), self.board, is_enpassant_move=True))
            if c < len(self.board[r]) - 1:
                if not piece_pinned or pin_direction == (1, 1):
                    if self.board[r+1][c+1][0] == 'w': # right capture
                        moves.append(Move((r,c), (r+1, c+1), self.board))
                    elif (r+1, c+1) == self.enpassant_possible: # right enpassent
                        is_attacking_piece = is_blocking_piece = False
                        if king_r == r:
                            if king_c < c: # king is left of pawn
                                # inside between king and pawn, outside range between pawn and border
                                inside_range = range(king_c+1, c)
                                outside_range = range(c+2, 8)
                            else: # king is right of pawn
                                inside_range = range(king_c-1, c+1, -1)
                                outside_range = range(c-1, -1, -1)
                            for i in inside_range:
                                if self.board[r][i] != '--':
                                    is_blocking_piece = True
                            for i in outside_range:
                                sq = self.board[r][i]
                                if sq[0] == 'w' and (sq[1] == 'R' or sq[1] == 'Q'): # attacking piece
                                    is_attacking_piece = True
                                elif sq != '--':
                                    is_blocking_piece = True
                        if not is_attacking_piece or is_blocking_piece:
                            moves.append(Move((r,c), (r+1, c+1), self.board, is_enpassant_move=True))


    def getRookMoves(self, r, c, moves) -> None:
        piece_pinned = False
        pin_direction = ()
        for i in range(len(self.pins)-1, -1, -1):
            if self.pins[i][0] == r and self.pins[i][1] == c:
                piece_pinned = True
                pin_direction = (self.pins[i][2], self.pins[i][3])
                if self.board[r][c][1] != 'Q': # can't remove queen from pin on rock moves, only remove on bishop moves
                    self.pins.remove(self.pins[i])
                break

        enemy_color = 'b' if self.whites_turn else 'w'
        dirs = ((-1, 0), (1, 0), (0, 1), (0, -1))
        for d in dirs:
            for i in range(1, len(self.board)):
                end_r = r + d[0] * i
                end_c = c + d[1] * i
                if 0 <= end_r < len(self.board) and 0 <= end_c < len(self.board[r]):
                    if not piece_pinned or pin_direction == d or pin_direction == (-d[0], -d[1]):
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
        piece_pinned = False
        for i in range(len(self.pins)-1, -1, -1):
            if self.pins[i][0] == r and self.pins[i][1] == c:
                piece_pinned = True
                self.pins.remove(self.pins[i])
                break

        enemy_color = 'b' if self.whites_turn else 'w'
        dirs = ((-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1))
        for d in dirs:
            end_r = r + d[0]
            end_c = c + d[1]
            if 0 <= end_r < len(self.board) and 0 <= end_c < len(self.board[r]):
                if not piece_pinned:
                    end_piece = self.board[end_r][end_c]
                    if end_piece == '--' or end_piece[0] == enemy_color:
                        moves.append(Move((r,c), (end_r, end_c), self.board))


    def getBishopMoves(self, r, c, moves) -> None:
        piece_pinned = False
        pin_direction = ()
        for i in range(len(self.pins)-1, -1, -1):
            if self.pins[i][0] == r and self.pins[i][1] == c:
                piece_pinned = True
                pin_direction = (self.pins[i][2], self.pins[i][3])
                self.pins.remove(self.pins[i])
                break

        # TODO: try combining with rook moves
        enemy_color = 'b' if self.whites_turn else 'w'
        dirs = ((-1, -1), (1, 1), (-1, 1), (1, -1))
        for d in dirs:
            for i in range(1, len(self.board)):
                end_r = r + d[0] * i
                end_c = c + d[1] * i
                if 0 <= end_r < len(self.board) and 0 <= end_c < len(self.board[r]):
                    if not piece_pinned or pin_direction == d or pin_direction == (-d[0], -d[1]):
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
        ally_color = 'w' if self.whites_turn else 'b'
        dirs = ((-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1))
        for i in range(len(dirs)):
            end_r = r + dirs[i][0]
            end_c = c + dirs[i][1]
            if 0 <= end_r < len(self.board) and 0 <= end_c < len(self.board[r]):
                end_piece = self.board[end_r][end_c]
                if end_piece[0] != ally_color:
                    # place king on end square and check for checks
                    if ally_color == 'w':
                        self.wk_location = (end_r, end_c)
                    else:
                        self.bk_location = (end_r, end_c)
                    is_in_check, _, _ = self.checkForPinsAndChecks()
                    if not is_in_check:
                        moves.append(Move((r,c), (end_r, end_c), self.board))
                    # place king back on original square
                    if ally_color == 'w':
                        self.wk_location = (r, c)
                    else:
                        self.bk_location = (r, c)


    """
    Generate all valid castle moves for the king at (r, c) and add to list of moves
    """
    def getCastleMoves(self, r, c, moves) -> None:
        if self.squareUnderAttack(r, c):
            return
        if (self.whites_turn and self.current_castling_rights.wks) or (not self.whites_turn and self.current_castling_rights.bks):
            self.getKingSideCastleMoves(r, c, moves)
        if (self.whites_turn and self.current_castling_rights.wqs) or (not self.whites_turn and self.current_castling_rights.bqs):
            self.getQueenSideCastleMoves(r, c, moves)


    def getKingSideCastleMoves(self, r, c, moves) -> None:
        if self.board[r][c+1] == '--' and self.board[r][c+2] == '--':
            if not self.squareUnderAttack(r, c+1) and not self.squareUnderAttack(r, c+2):
                moves.append(Move((r, c), (r, c+2), self.board, is_castle_move=True))


    def getQueenSideCastleMoves(self, r, c, moves) -> None:
        if self.board[r][c-1] == '--' and self.board[r][c-2] == '--' and self.board[r][c-3] == '--':
            if not self.squareUnderAttack(r, c-1) and not self.squareUnderAttack(r, c-2):
                moves.append(Move((r, c), (r, c-2), self.board, is_castle_move=True))


class CastleRights():
    def __init__(self, wks, bks, wqs, bqs) -> None:
        self.wks = wks
        self.bks = bks
        self.wqs = wqs
        self.bqs = bqs


class Move():
    # the following converts between cordinate and rank notation
    rank_to_rows = {'1': 7, '2': 6, '3': 5, '4': 4,
                    '5': 3, '6': 2, '7': 1, '8': 0}
    rows_to_ranks = {v: k for k, v in rank_to_rows.items()}
    files_to_cols = {'a': 0, 'b': 1, 'c': 2, 'd': 3,
                     'e': 4, 'f': 5, 'g': 6, 'h': 7}
    cols_to_files = {v: k for k, v in files_to_cols.items()}


    def __init__(self, start_sq, end_sq, board, is_enpassant_move=False, is_castle_move=False) -> None:
        self.start_r, self.start_c = start_sq[0], start_sq[1]
        self.end_r, self.end_c = end_sq[0], end_sq[1]
        self.piece_moved = board[self.start_r][self.start_c]
        self.piece_captured = board[self.end_r][self.end_c]
        # pawn promotion
        self.is_pawn_promo = (self.piece_moved == 'wP' and self.end_r == 0) or (self.piece_moved == 'bP' and self.end_r == 7)
        # enpassant
        self.is_enpassant_move = is_enpassant_move
        if self.is_enpassant_move:
            self.piece_captured = 'wP' if self.piece_moved == 'bP' else 'bP'
        # castling
        self.is_castle_move = is_castle_move

        self.is_capture = self.piece_captured != '--'
        self.move_id = self.start_r*1000 + self.start_c*100 + self.end_r*10 + self.end_c


    def __eq__(self, value: object) -> bool:
        return isinstance(value, Move) and (self.move_id == value.move_id)


    def getChessNotation(self) -> str:
        return f'{self.getRankFile(self.start_r, self.start_c)}{self.getRankFile(self.end_r, self.end_c)}'


    def getRankFile(self, r, c) -> str:
        return f'{self.cols_to_files[c]}{self.rows_to_ranks[r]}'


    def __str__(self) -> str:
        # castle move
        if self.is_castle_move:
            return 'O-O' if self.end_c == 6 else 'O-O-O'

        end_sq = self.getRankFile(self.end_r, self.end_c)

        # pawn moves
        if self.piece_moved[1] == 'P':
            if self.is_capture:
                return self.cols_to_files[self.start_c] + 'x' + end_sq
            else:
                return end_sq

        # piece moves
        move_str = self.piece_moved[1]
        if self.is_capture:
            move_str += 'x'
        return move_str + end_sq
