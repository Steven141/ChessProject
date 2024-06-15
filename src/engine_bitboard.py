"""
Bitboard Engine
"""


import pdb
from bitboard import BitBoard, BitBoardMasks


PIECE_NAMES = ['wP', 'wN', 'wB', 'wR', 'wQ', 'wK', 'bP', 'bN', 'bB', 'bR', 'bQ', 'bK']
BB_1 = BitBoard(1)


class GameState():
    """
    In this 8x8 game board, the first char represents color
    and the second char represents type of piece. Empty
    square is denoted with '--'.
    """
    def __init__(self) -> None:
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

        self.initBitBoards()
        # castling variables
        self.cwK = self.cwQ = self.cbK = self.cbQ = True
        self.whites_turn = True
        # populate bitboards
        self.arrayToBitboard()
        self.masks = BitBoardMasks()


    """
    Initialize each piece bitboard and bitboard for enpassant
    """
    def initBitBoards(self) -> None:
        for piece in PIECE_NAMES:
            setattr(self, piece, BitBoard())
        self.EP = BitBoard() # zero if no enpassant can happen


    """
    Populate piece bitboards from array representation of game board
    """
    def arrayToBitboard(self) -> None:
        for i in range(64):
            binary = '0000000000000000000000000000000000000000000000000000000000000000'
            binary = binary[:i] + '1' + binary[i+1:]
            match self.board[i // 8][i % 8]:
                case 'wP': self.wP += BitBoard(bin_str=binary)
                case 'wN': self.wN += BitBoard(bin_str=binary)
                case 'wB': self.wB += BitBoard(bin_str=binary)
                case 'wR': self.wR += BitBoard(bin_str=binary)
                case 'wQ': self.wQ += BitBoard(bin_str=binary)
                case 'wK': self.wK += BitBoard(bin_str=binary)
                case 'bP': self.bP += BitBoard(bin_str=binary)
                case 'bN': self.bN += BitBoard(bin_str=binary)
                case 'bB': self.bB += BitBoard(bin_str=binary)
                case 'bR': self.bR += BitBoard(bin_str=binary)
                case 'bQ': self.bQ += BitBoard(bin_str=binary)
                case 'bK': self.bK += BitBoard(bin_str=binary)


    """
    Prints the current state of the game
    """
    def drawGameArray(self) -> None:
        new_board = [['--']*8 for _ in range(8)]
        for i in range(64): # i = 0 -> board[0][0] -> bitboard_as_bin[0] -> top left
            shift = 64 - 1 - i
            if (self.wP >> shift) & 1 == 1:
                new_board[i // 8][i % 8] = 'wP'
            if (self.wN >> shift) & 1 == 1:
                new_board[i // 8][i % 8] = 'wN'
            if (self.wB >> shift) & 1 == 1:
                new_board[i // 8][i % 8] = 'wB'
            if (self.wR >> shift) & 1 == 1:
                new_board[i // 8][i % 8] = 'wR'
            if (self.wQ >> shift) & 1 == 1:
                new_board[i // 8][i % 8] = 'wQ'
            if (self.wK >> shift) & 1 == 1:
                new_board[i // 8][i % 8] = 'wK'
            if (self.bP >> shift) & 1 == 1:
                new_board[i // 8][i % 8] = 'bP'
            if (self.bN >> shift) & 1 == 1:
                new_board[i // 8][i % 8] = 'bN'
            if (self.bB >> shift) & 1 == 1:
                new_board[i // 8][i % 8] = 'bB'
            if (self.bR >> shift) & 1 == 1:
                new_board[i // 8][i % 8] = 'bR'
            if (self.bQ >> shift) & 1 == 1:
                new_board[i // 8][i % 8] = 'bQ'
            if (self.bK >> shift) & 1 == 1:
                new_board[i // 8][i % 8] = 'bK'
        for r in new_board:
            print(*r)


    """
    Set the state of the bitboards to the FEN string
    """
    def importFEN(self, fen_str) -> None:
        self.initBitBoards()
        self.cwK = self.cwQ = self.cbK = self.cbQ = False
        char_idx = 0
        board_idx = 0
        while fen_str[char_idx] != ' ':
            board_idx_shift = 64 - 1 - board_idx
            match fen_str[char_idx]:
                case 'P':
                    self.wP |= (BB_1 << board_idx_shift)
                    board_idx += 1
                case 'p':
                    self.bP |= (BB_1 << board_idx_shift)
                    board_idx += 1
                case 'N':
                    self.wN |= (BB_1 << board_idx_shift)
                    board_idx += 1
                case 'n':
                    self.bN |= (BB_1 << board_idx_shift)
                    board_idx += 1
                case 'B':
                    self.wB |= (BB_1 << board_idx_shift)
                    board_idx += 1
                case 'b':
                    self.bB |= (BB_1 << board_idx_shift)
                    board_idx += 1
                case 'R':
                    self.wR |= (BB_1 << board_idx_shift)
                    board_idx += 1
                case 'r':
                    self.bR |= (BB_1 << board_idx_shift)
                    board_idx += 1
                case 'Q':
                    self.wQ |= (BB_1 << board_idx_shift)
                    board_idx += 1
                case 'q':
                    self.bQ |= (BB_1 << board_idx_shift)
                    board_idx += 1
                case 'K':
                    self.wK |= (BB_1 << board_idx_shift)
                    board_idx += 1
                case 'k':
                    self.bK |= (BB_1 << board_idx_shift)
                    board_idx += 1
                case '1': board_idx += 1
                case '2': board_idx += 2
                case '3': board_idx += 3
                case '4': board_idx += 4
                case '5': board_idx += 5
                case '6': board_idx += 6
                case '7': board_idx += 7
                case '8': board_idx += 8
            char_idx += 1

        char_idx += 1
        self.whites_turn = fen_str[char_idx] == 'w'
        char_idx += 2

        while fen_str[char_idx] != ' ':
            match fen_str[char_idx]:
                case 'K': self.cwK = True
                case 'Q': self.cwQ = True
                case 'k': self.cbK = True
                case 'q': self.cbQ = True
            char_idx += 1

        char_idx += 1
        if fen_str[char_idx] != '-':
            self.EP = self.masks.file_masks[ord(fen_str[char_idx]) - ord('a')]
            char_idx += 1
        # rest of FEN not used


class Moves():
    def __init__(self) -> None:
        self.castle_rooks = [63, 56, 7, 0]
        self.masks = BitBoardMasks()


    """
    Takes in a piece bitboard, move string, and piece type and returns resulting piece bitboard
    """
    def makeMove(self, bitboard, move, p_type) -> int:
        if move[3].isnumeric(): # regular move
            start_shift = 64 - 1 - (int(move[0]) * 8 + int(move[1]))
            end_shift = 64 - 1 - (int(move[2]) * 8 + int(move[3]))
            if (bitboard >> start_shift) & 1 == 1:
                bitboard &= ~(BB_1 << start_shift) # remove moving piece from board
                bitboard |= (BB_1 << end_shift) # add at new position
            else:
                bitboard &= ~(BB_1 << end_shift) # remove piece at end

        elif move[3] == 'P': # pawn promo
            if move[2].isupper(): # white promo
                start_bitboard = self.masks.file_masks[int(move[0])] & self.masks.rank_masks[1]
                start_shift = 64 - 1 - start_bitboard.asBinaryString().index('1')
                end_bitboard = self.masks.file_masks[int(move[1])] & self.masks.rank_masks[0]
                end_shift = 64 - 1 - end_bitboard.asBinaryString().index('1')
            else: # black promo
                start_bitboard = self.masks.file_masks[int(move[0])] & self.masks.rank_masks[6]
                start_shift = 64 - 1 - start_bitboard.asBinaryString().index('1')
                end_bitboard = self.masks.file_masks[int(move[1])] & self.masks.rank_masks[7]
                end_shift = 64 - 1 - end_bitboard.asBinaryString().index('1')
            if p_type == move[2]:
                bitboard |= (BB_1 << end_shift)
            else:
                bitboard &= ~(BB_1 << start_shift)
                bitboard &= ~(BB_1 << end_shift)

        elif move[3] == 'E': # enpassant
            if move[2] == 'w': # white
                start_bitboard = self.masks.file_masks[int(move[0])] & self.masks.rank_masks[3]
                start_shift = 64 - 1 - start_bitboard.asBinaryString().index('1')
                end_bitboard = self.masks.file_masks[int(move[1])] & self.masks.rank_masks[2]
                end_shift = 64 - 1 - end_bitboard.asBinaryString().index('1')
                bitboard &= ~(self.masks.file_masks[int(move[1])] & self.masks.rank_masks[3])
            else: # black
                start_bitboard = self.masks.file_masks[int(move[0])] & self.masks.rank_masks[4]
                start_shift = 64 - 1 - start_bitboard.asBinaryString().index('1')
                end_bitboard = self.masks.file_masks[int(move[1])] & self.masks.rank_masks[5]
                end_shift = 64 - 1 - end_bitboard.asBinaryString().index('1')
                bitboard &= ~(self.masks.file_masks[int(move[1])] & self.masks.rank_masks[4])
            if (bitboard >> start_shift) & 1 == 1:
                bitboard &= ~(BB_1 << start_shift)
                bitboard |= (BB_1 << end_shift)
        else:
            raise ValueError('INVALID MOVE TYPE')

        return bitboard


    """
    Takes in a rook / king bitboards, move string, and piece type and returns resulting rook bitboard
    """
    def makeMoveCastle(self, rook, king, move, p_type) -> int:
        start_shift = 64 - 1 - (int(move[0]) * 8 + int(move[1]))
        if ((king >> start_shift) & 1 == 1) and ((move == '0402') or (move == '0406') or (move == '7472') or (move == '7476')):
            if p_type == 'R': # white
                match move:
                    case '7476': # king side
                        rook &= ~(BB_1 << self.castle_rooks[3])
                        rook |= (BB_1 << (self.castle_rooks[3] + 2))
                    case '7472': # queen side
                        rook &= ~(BB_1 << self.castle_rooks[2])
                        rook |= (BB_1 << (self.castle_rooks[2] - 3))
            else: # black
                match move:
                    case '0406': # king side
                        rook &= ~(BB_1 << self.castle_rooks[1])
                        rook |= (BB_1 << (self.castle_rooks[1] + 2))
                    case '0402': # queen side
                        rook &= ~(BB_1 << self.castle_rooks[0])
                        rook |= (BB_1 << (self.castle_rooks[0] - 3))
        return rook


    """
    Return biboard of file where enpassant possible
    """
    def makeMoveEP(self, bitboard, move) -> int:
        start_shift = 64 - 1 - (int(move[0]) * 8 + int(move[1]))
        if move[3].isnumeric() and (abs(int(move[0]) - int(move[2])) == 2) and (((bitboard >> start_shift) & 1) == 1):
            return self.masks.file_masks[int(move[1])]
        return BitBoard(0)


    """
    Return a move list string containing all possible moves for white
    """
    def possibleMovesW(self, wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK, EP, cwK, cwQ, cbK, cbQ) -> str:
        self.masks.not_allied_pieces = ~(wP|wN|wB|wR|wQ|wK|bK) # avoid illegal bK capture
        self.masks.enemy_pieces = bP|bN|bB|bR|bQ # avoid illegal bK capture
        self.masks.empty = ~(wP|wN|wB|wR|wQ|wK|bP|bN|bB|bR|bQ|bK)
        self.masks.occupied = ~self.masks.empty
        move_list = self.possibleWP(wP, bP, EP) + \
            self.possibleB(wB) + self.possibleQ(wQ) + \
            self.possibleR(wR) + self.possibleN(wN) + \
            self.possibleK(wK) + self.possibleCastleW(wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK, cwK, cwQ)
        return move_list


    """
    Return a move list string containing all possible moves for black
    """
    def possibleMovesB(self, wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK, EP, cwK, cwQ, cbK, cbQ) -> str:
        self.masks.not_allied_pieces = ~(bP|bN|bB|bR|bQ|bK|wK) # avoid illegal wK capture
        self.masks.enemy_pieces = wP|wN|wB|wR|wQ # avoid illegal wK capture
        self.masks.empty = ~(wP|wN|wB|wR|wQ|wK|bP|bN|bB|bR|bQ|bK)
        self.masks.occupied = ~self.masks.empty
        move_list = self.possibleBP(wP, bP, EP) + \
            self.possibleB(bB) + self.possibleQ(bQ) + \
            self.possibleR(bR) + self.possibleN(bN) + \
            self.possibleK(bK) + self.possibleCastleB(wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK, cbK, cbQ)
        return move_list


    """
    Return a move list string containing all possible moves for a white pawn
    """
    def possibleWP(self, wP, bP, EP) -> str:
        # standard moves and captures
        move_list = '' # r1,c1,r2,c2
        moves = ((wP << 7) & self.masks.enemy_pieces & ~self.masks.rank_masks[0] & ~self.masks.file_masks[0]) # right capture
        possible_move = (moves & ~(moves - 1)) # selects single possible move
        while possible_move != 0:
            idx = possible_move.asBinaryString().index('1')
            move_list += f'{(idx // 8) + 1}{(idx % 8) - 1}{idx // 8}{idx % 8}'
            moves &= ~possible_move # remove current move from moves
            possible_move = (moves & ~(moves - 1)) # get next possible move

        moves = ((wP << 9) & self.masks.enemy_pieces & ~self.masks.rank_masks[0] & ~self.masks.file_masks[7]) # left capture
        possible_move = (moves & ~(moves - 1))
        while possible_move != 0:
            idx = possible_move.asBinaryString().index('1')
            move_list += f'{(idx // 8) + 1}{(idx % 8) + 1}{idx // 8}{idx % 8}'
            moves &= ~possible_move
            possible_move = (moves & ~(moves - 1))

        moves = ((wP << 8) & self.masks.empty & ~self.masks.rank_masks[0]) # move forward 1
        possible_move = (moves & ~(moves - 1))
        while possible_move != 0:
            idx = possible_move.asBinaryString().index('1')
            move_list += f'{(idx // 8) + 1}{idx % 8}{idx // 8}{idx % 8}'
            moves &= ~possible_move
            possible_move = (moves & ~(moves - 1))

        moves = ((wP << 16) & self.masks.empty & (self.masks.empty << 8) & self.masks.rank_masks[4]) # move forward 2
        possible_move = (moves & ~(moves - 1))
        while possible_move != 0:
            idx = possible_move.asBinaryString().index('1')
            move_list += f'{(idx // 8) + 2}{idx % 8}{idx // 8}{idx % 8}'
            moves &= ~possible_move
            possible_move = (moves & ~(moves - 1))

        # pawn promotion, move_list -> c1,c2,promo type,'P'
        moves = ((wP << 7) & self.masks.enemy_pieces & self.masks.rank_masks[0] & ~self.masks.file_masks[0]) # promo by right capture
        possible_move = (moves & ~(moves - 1))
        while possible_move != 0:
            idx = possible_move.asBinaryString().index('1')
            c1, c2 = (idx % 8) - 1, idx % 8
            move_list += f'{c1}{c2}QP{c1}{c2}RP{c1}{c2}BP{c1}{c2}NP'
            moves &= ~possible_move
            possible_move = (moves & ~(moves - 1))

        moves = ((wP << 9) & self.masks.enemy_pieces & self.masks.rank_masks[0] & ~self.masks.file_masks[7]) # promo by left capture
        possible_move = (moves & ~(moves - 1))
        while possible_move != 0:
            idx = possible_move.asBinaryString().index('1')
            c1, c2 = (idx % 8) + 1, idx % 8
            move_list += f'{c1}{c2}QP{c1}{c2}RP{c1}{c2}BP{c1}{c2}NP'
            moves &= ~possible_move
            possible_move = (moves & ~(moves - 1))

        moves = ((wP << 8) & self.masks.empty & self.masks.rank_masks[0]) # promo by move forward 1
        possible_move = (moves & ~(moves - 1))
        while possible_move != 0:
            idx = possible_move.asBinaryString().index('1')
            c1 = c2 = idx % 8
            move_list += f'{c1}{c2}QP{c1}{c2}RP{c1}{c2}BP{c1}{c2}NP'
            moves &= ~possible_move
            possible_move = (moves & ~(moves - 1))

        # enpassant, move_list -> c1,c2,'wE'
        moves = ((wP >> 1) & bP & self.masks.rank_masks[3] & ~self.masks.file_masks[0] & EP) # enpassant right
        possible_move = (moves & ~(moves - 1))
        while possible_move != 0:
            idx = possible_move.asBinaryString().index('1')
            c1, c2 = (idx % 8) - 1, idx % 8
            move_list += f'{c1}{c2}wE'
            moves &= ~possible_move
            possible_move = (moves & ~(moves - 1))

        moves = ((wP << 1) & bP & self.masks.rank_masks[3] & ~self.masks.file_masks[7] & EP) # enpassant left
        possible_move = (moves & ~(moves - 1))
        while possible_move != 0:
            idx = possible_move.asBinaryString().index('1')
            c1, c2 = (idx % 8) + 1, idx % 8
            move_list += f'{c1}{c2}wE'
            moves &= ~possible_move
            possible_move = (moves & ~(moves - 1))

        return move_list


    """
    Return a move list string containing all possible moves for a black pawn
    """
    def possibleBP(self, wP, bP, EP) -> str:
        # standard moves and captures
        move_list = '' # r1,c1,r2,c2
        moves = ((bP >> 7) & self.masks.enemy_pieces & ~self.masks.rank_masks[7] & ~self.masks.file_masks[7]) # right capture
        possible_move = (moves & ~(moves - 1)) # selects single possible move
        while possible_move != 0:
            idx = possible_move.asBinaryString().index('1')
            move_list += f'{(idx // 8) - 1}{(idx % 8) + 1}{idx // 8}{idx % 8}'
            moves &= ~possible_move # remove current move from moves
            possible_move = (moves & ~(moves - 1)) # get next possible move

        moves = ((bP >> 9) & self.masks.enemy_pieces & ~self.masks.rank_masks[7] & ~self.masks.file_masks[0]) # left capture
        possible_move = (moves & ~(moves - 1))
        while possible_move != 0:
            idx = possible_move.asBinaryString().index('1')
            move_list += f'{(idx // 8) - 1}{(idx % 8) - 1}{idx // 8}{idx % 8}'
            moves &= ~possible_move
            possible_move = (moves & ~(moves - 1))

        moves = ((bP >> 8) & self.masks.empty & ~self.masks.rank_masks[7]) # move forward 1
        possible_move = (moves & ~(moves - 1))
        while possible_move != 0:
            idx = possible_move.asBinaryString().index('1')
            move_list += f'{(idx // 8) - 1}{idx % 8}{idx // 8}{idx % 8}'
            moves &= ~possible_move
            possible_move = (moves & ~(moves - 1))

        moves = ((bP >> 16) & self.masks.empty & (self.masks.empty >> 8) & self.masks.rank_masks[3]) # move forward 2
        possible_move = (moves & ~(moves - 1))
        while possible_move != 0:
            idx = possible_move.asBinaryString().index('1')
            move_list += f'{(idx // 8) - 2}{idx % 8}{idx // 8}{idx % 8}'
            moves &= ~possible_move
            possible_move = (moves & ~(moves - 1))

        # pawn promotion, move_list -> c1,c2,promo type,'P'
        moves = ((bP >> 7) & self.masks.enemy_pieces & self.masks.rank_masks[7] & ~self.masks.file_masks[7]) # promo by right capture
        possible_move = (moves & ~(moves - 1))
        while possible_move != 0:
            idx = possible_move.asBinaryString().index('1')
            c1, c2 = (idx % 8) + 1, idx % 8
            move_list += f'{c1}{c2}qP{c1}{c2}rP{c1}{c2}bP{c1}{c2}nP'
            moves &= ~possible_move
            possible_move = (moves & ~(moves - 1))

        moves = ((bP >> 9) & self.masks.enemy_pieces & self.masks.rank_masks[7] & ~self.masks.file_masks[0]) # promo by left capture
        possible_move = (moves & ~(moves - 1))
        while possible_move != 0:
            idx = possible_move.asBinaryString().index('1')
            c1, c2 = (idx % 8) - 1, idx % 8
            move_list += f'{c1}{c2}qP{c1}{c2}rP{c1}{c2}bP{c1}{c2}nP'
            moves &= ~possible_move
            possible_move = (moves & ~(moves - 1))

        moves = ((bP >> 8) & self.masks.empty & self.masks.rank_masks[7]) # promo by move forward 1
        possible_move = (moves & ~(moves - 1))
        while possible_move != 0:
            idx = possible_move.asBinaryString().index('1')
            c1 = c2 = idx % 8
            move_list += f'{c1}{c2}qP{c1}{c2}rP{c1}{c2}bP{c1}{c2}nP'
            moves &= ~possible_move
            possible_move = (moves & ~(moves - 1))

        # enpassant, move_list -> c1,c2,'bE'
        moves = ((bP << 1) & wP & self.masks.rank_masks[4] & ~self.masks.file_masks[7] & EP) # enpassant right
        possible_move = (moves & ~(moves - 1))
        while possible_move != 0:
            idx = possible_move.asBinaryString().index('1')
            c1, c2 = (idx % 8) + 1, idx % 8
            move_list += f'{c1}{c2}bE'
            moves &= ~possible_move
            possible_move = (moves & ~(moves - 1))

        moves = ((bP >> 1) & wP & self.masks.rank_masks[4] & ~self.masks.file_masks[0] & EP) # enpassant left
        possible_move = (moves & ~(moves - 1))
        while possible_move != 0:
            idx = possible_move.asBinaryString().index('1')
            c1, c2 = (idx % 8) - 1, idx % 8
            move_list += f'{c1}{c2}bE'
            moves &= ~possible_move
            possible_move = (moves & ~(moves - 1))

        return move_list


    """
    Return a move list string containing all possible moves for a bishop
    """
    def possibleB(self, B) -> str:
        move_list = ''
        bishop = (B & ~(B - 1))

        while bishop != 0:
            bishop_idx = bishop.asBinaryString().index('1')
            moves = self.possibleDiagAndAntiDiagMoves(bishop_idx) & self.masks.not_allied_pieces
            possible_move = (moves & ~(moves - 1)) # selects single possible move

            while possible_move != 0:
                move_idx = possible_move.asBinaryString().index('1')
                move_list += f'{bishop_idx // 8}{bishop_idx % 8}{move_idx // 8}{move_idx % 8}'
                moves &= ~possible_move # remove current possible move
                possible_move = (moves & ~(moves - 1))

            B &= ~bishop # remove current bishop
            bishop = (B & ~(B - 1))

        return move_list


    """
    Return a move list string containing all possible moves for a queen
    """
    def possibleQ(self, Q) -> str:
        move_list = ''
        queen = (Q & ~(Q - 1))

        while queen != 0:
            queen_idx = queen.asBinaryString().index('1')
            moves = (self.possibleDiagAndAntiDiagMoves(queen_idx) | self.possibleHAndVMoves(queen_idx)) & self.masks.not_allied_pieces
            possible_move = (moves & ~(moves - 1)) # selects single possible move

            while possible_move != 0:
                move_idx = possible_move.asBinaryString().index('1')
                move_list += f'{queen_idx // 8}{queen_idx % 8}{move_idx // 8}{move_idx % 8}'
                moves &= ~possible_move # remove current possible move
                possible_move = (moves & ~(moves - 1))

            Q &= ~queen # remove current queen
            queen = (Q & ~(Q - 1))

        return move_list


    """
    Return a move list string containing all possible moves for a rook
    """
    def possibleR(self, R) -> str:
        move_list = ''
        rook = (R & ~(R - 1))

        while rook != 0:
            rook_idx = rook.asBinaryString().index('1')
            moves = self.possibleHAndVMoves(rook_idx) & self.masks.not_allied_pieces
            possible_move = (moves & ~(moves - 1)) # selects single possible move

            while possible_move != 0:
                move_idx = possible_move.asBinaryString().index('1')
                move_list += f'{rook_idx // 8}{rook_idx % 8}{move_idx // 8}{move_idx % 8}'
                moves &= ~possible_move # remove current possible move
                possible_move = (moves & ~(moves - 1))

            R &= ~rook # remove current rook
            rook = (R & ~(R - 1))

        return move_list


    """
    Return a move list string containing all possible moves for a knight
    """
    def possibleN(self, N) -> str:
        move_list = ''
        knight = (N & ~(N - 1))
        knight_span_c6_idx = 18

        while knight != 0:
            knight_idx = knight.asBinaryString().index('1')

            # allign the knight_span_c6 mask
            if knight_idx <= knight_span_c6_idx:
                moves = self.masks.knight_span_c6 << (knight_span_c6_idx - knight_idx)
            else:
                moves = self.masks.knight_span_c6 >> (knight_idx - knight_span_c6_idx)

            # remove moves sliding off board or allied pieces
            if knight_idx % 8 < 4:
                moves &= (~self.masks.file_gh) & self.masks.not_allied_pieces
            else:
                moves &= (~self.masks.file_ab) & self.masks.not_allied_pieces
            possible_move = (moves & ~(moves - 1)) # selects single possible move

            while possible_move != 0:
                move_idx = possible_move.asBinaryString().index('1')
                move_list += f'{knight_idx // 8}{knight_idx % 8}{move_idx // 8}{move_idx % 8}'
                moves &= ~possible_move # remove current possible move
                possible_move = (moves & ~(moves - 1))

            N &= ~knight # remove current knight
            knight = (N & ~(N - 1))

        return move_list


    """
    Return a move list string containing all possible moves for a king
    """
    def possibleK(self, K) -> str:
        move_list = ''
        king = (K & ~(K - 1))
        king_span_c7_idx = 10

        while king != 0:
            king_idx = king.asBinaryString().index('1')

            # allign the king_span_c7 mask
            if king_idx <= king_span_c7_idx:
                moves = self.masks.king_span_c7 << (king_span_c7_idx - king_idx)
            else:
                moves = self.masks.king_span_c7 >> (king_idx - king_span_c7_idx)

            # remove moves sliding off board or allied pieces
            if king_idx % 8 < 4:
                moves &= (~self.masks.file_gh) & self.masks.not_allied_pieces
            else:
                moves &= (~self.masks.file_ab) & self.masks.not_allied_pieces
            possible_move = (moves & ~(moves - 1)) # selects single possible move

            while possible_move != 0:
                move_idx = possible_move.asBinaryString().index('1')
                move_list += f'{king_idx // 8}{king_idx % 8}{move_idx // 8}{move_idx % 8}'
                moves &= ~possible_move # remove current possible move
                possible_move = (moves & ~(moves - 1))

            K &= ~king # remove current king
            king = (K & ~(K - 1))

        return move_list


    """
    Return a move list string containing all possible castles for white
    """
    def possibleCastleW(self, wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK, cwK, cwQ) -> str:
        unsafe = self.unsafeForWhite(wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK)
        move_list = '' # king move r1c1r2c1
        if (unsafe & wK) == 0:
            if cwK and (((BB_1 << self.castle_rooks[3]) & wR) != 0):
                if ((self.masks.occupied | unsafe) & ((BB_1 << 1) | (BB_1 << 2))) == 0:
                    move_list += '7476'
            if cwQ and (((BB_1 << self.castle_rooks[2]) & wR) != 0):
                if ((self.masks.occupied | (unsafe & ~(BB_1 << 6))) & ((BB_1 << 4) | (BB_1 << 5) | (BB_1 << 6))) == 0:
                    move_list += '7472'
        return move_list


    """
    Return a move list string containing all possible castles for black
    """
    def possibleCastleB(self, wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK, cbK, cbQ) -> str:
        unsafe = self.unsafeForBlack(wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK)
        move_list = '' # king move r1c1r2c1
        if (unsafe & bK) == 0:
            if cbK and (((BB_1 << self.castle_rooks[1]) & bR) != 0):
                if ((self.masks.occupied | unsafe) & ((BB_1 << 58) | (BB_1 << 57))) == 0:
                    move_list += '0406'
            if cbQ and (((BB_1 << self.castle_rooks[0]) & bR) != 0):
                if ((self.masks.occupied | (unsafe & ~(BB_1 << 62))) & ((BB_1 << 62) | (BB_1 << 61) | (BB_1 << 60))) == 0:
                    move_list += '0402'
        return move_list


    """
    Returns all possible horizontal and vertical moves of piece at index piece_idx

    Example for formula derivation:
    occupied = o = 11000101 -> wP wP -- -- -- bR -- wP
    slider = s = 00000100
    o - s = 11000001 -> removes slider bit
    o - 2s = 10111101 -> flips bits left of slider bit until first seen occupied bit (inclusive)
    left = o^(o-2s) = 01111000 -> extracts all possible left sliding positions including first taken piece
    let o' denote reverse of o
    right = (o'^(o'-2s'))' = o^(o'-2s')' = 00000011
    lineAttacks_h = right^left = o^(o'-2s')' ^ o^(o-2s) = (o'-2s')' ^ (o-2s)
    m = mask
    lineAttacks_v = (((o&m)'-2s')' ^ ((o&m)-2s))

    return (possible_h & rank_m) | (possible_v & file_m) to only consider one file and rank
    """
    def possibleHAndVMoves(self, piece_idx) -> int:
        # piece_idx = 0 -> top left of board -> 1000...000
        binary_idx = BB_1 << (64 - 1 - piece_idx)
        rank_mask = self.masks.rank_masks[piece_idx // 8]
        file_mask = self.masks.file_masks[piece_idx % 8]

        possible_h = (self.masks.occupied - binary_idx*2) ^ (self.masks.occupied.reverseBits() - binary_idx.reverseBits()*2).reverseBits()
        possible_v = ((self.masks.occupied & file_mask) - binary_idx*2) ^ ((self.masks.occupied & file_mask).reverseBits() - binary_idx.reverseBits()*2).reverseBits()
        return (possible_h & rank_mask) | (possible_v & file_mask)


    """
    Returns all possible diagonal and anti-diagonal moves of piece at index piece_idx

    See possibleHAndVMoves func description for formula derivation
    """
    def possibleDiagAndAntiDiagMoves(self, piece_idx) -> int:
        # piece_idx = 0 -> top left of board -> 1000...000
        binary_idx = BB_1 << (64 - 1 - piece_idx)
        diag_mask = self.masks.diagonal_masks[(piece_idx // 8) + (piece_idx % 8)]
        a_diag_mask = self.masks.anti_diagonal_masks[(piece_idx // 8) - (piece_idx % 8) + 7]

        possible_d = ((self.masks.occupied & diag_mask) - binary_idx*2) ^ ((self.masks.occupied & diag_mask).reverseBits() - binary_idx.reverseBits()*2).reverseBits()
        possible_ad = ((self.masks.occupied & a_diag_mask) - binary_idx*2) ^ ((self.masks.occupied & a_diag_mask).reverseBits() - binary_idx.reverseBits()*2).reverseBits()
        return (possible_d & diag_mask) | (possible_ad & a_diag_mask)


    """
    Returns a bitboard with 1's at all squares attacked by white
    """
    def unsafeForBlack(self, wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK) -> int:
        self.masks.occupied = wP|wN|wB|wR|wQ|wK|bP|bN|bB|bR|bQ|bK
        # pawn threats
        unsafe = ((wP << 7) & ~self.masks.file_masks[0]) # pawn right capture
        unsafe |= (((wP << 9) & ~self.masks.file_masks[7])) # pawn left capture

        # knight threat
        knight = (wN & ~(wN - 1))
        knight_span_c6_idx = 18
        while knight != 0:
            knight_idx = knight.asBinaryString().index('1')
            # allign the knight_span_c6 mask
            if knight_idx <= knight_span_c6_idx:
                moves = self.masks.knight_span_c6 << (knight_span_c6_idx - knight_idx)
            else:
                moves = self.masks.knight_span_c6 >> (knight_idx - knight_span_c6_idx)
            # remove moves sliding off board or allied pieces
            if knight_idx % 8 < 4:
                moves &= ~self.masks.file_gh
            else:
                moves &= ~self.masks.file_ab
            unsafe |= moves
            wN &= ~knight # remove current knight
            knight = (wN & ~(wN - 1))

        # bishop / queen threats (diagonals)
        wQB = wQ | wB
        b_or_q = (wQB & ~(wQB - 1))
        while b_or_q != 0:
            b_or_q_idx = b_or_q.asBinaryString().index('1')
            moves = self.possibleDiagAndAntiDiagMoves(b_or_q_idx)
            unsafe |= moves
            wQB &= ~b_or_q # remove current bishop or queen
            b_or_q = (wQB & ~(wQB - 1))

        # rook / queen threats (hor and vert)
        wQR = wQ | wR
        r_or_q = (wQR & ~(wQR - 1))
        while r_or_q != 0:
            r_or_q_idx = r_or_q.asBinaryString().index('1')
            moves = self.possibleHAndVMoves(r_or_q_idx)
            unsafe |= moves
            wQR &= ~r_or_q # remove current rook or queen
            r_or_q = (wQR & ~(wQR - 1))

        # king threats
        king = (wK & ~(wK - 1))
        king_span_c7_idx = 10
        while king != 0:
            king_idx = king.asBinaryString().index('1')
            # allign the king_span_c7 mask
            if king_idx <= king_span_c7_idx:
                moves = self.masks.king_span_c7 << (king_span_c7_idx - king_idx)
            else:
                moves = self.masks.king_span_c7 >> (king_idx - king_span_c7_idx)
            # remove moves sliding off board or allied pieces
            if king_idx % 8 < 4:
                moves &= ~self.masks.file_gh
            else:
                moves &= ~self.masks.file_ab
            unsafe |= moves
            wK &= ~king # remove current king
            king = (wK & ~(wK - 1))

        return unsafe


    """
    Returns a bitboard with 1's at all squares attacked by black
    """
    def unsafeForWhite(self, wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK) -> int:
        self.masks.occupied = wP|wN|wB|wR|wQ|wK|bP|bN|bB|bR|bQ|bK
        # pawn threats
        unsafe = ((bP >> 7) & ~self.masks.file_masks[7]) # pawn right capture
        unsafe |= (((bP >> 9) & ~self.masks.file_masks[0])) # pawn left capture

        # knight threat
        knight = (bN & ~(bN - 1))
        knight_span_c6_idx = 18
        while knight != 0:
            knight_idx = knight.asBinaryString().index('1')
            # allign the knight_span_c6 mask
            if knight_idx <= knight_span_c6_idx:
                moves = self.masks.knight_span_c6 << (knight_span_c6_idx - knight_idx)
            else:
                moves = self.masks.knight_span_c6 >> (knight_idx - knight_span_c6_idx)
            # remove moves sliding off board or allied pieces
            if knight_idx % 8 < 4:
                moves &= ~self.masks.file_gh
            else:
                moves &= ~self.masks.file_ab
            unsafe |= moves
            bN &= ~knight # remove current knight
            knight = (bN & ~(bN - 1))

        # bishop / queen threats (diagonals)
        bQB = bQ | bB
        b_or_q = (bQB & ~(bQB - 1))
        while b_or_q != 0:
            b_or_q_idx = b_or_q.asBinaryString().index('1')
            moves = self.possibleDiagAndAntiDiagMoves(b_or_q_idx)
            unsafe |= moves
            bQB &= ~b_or_q # remove current bishop or queen
            b_or_q = (bQB & ~(bQB - 1))

        # rook / queen threats (hor and vert)
        bQR = bQ | bR
        r_or_q = (bQR & ~(bQR - 1))
        while r_or_q != 0:
            r_or_q_idx = r_or_q.asBinaryString().index('1')
            moves = self.possibleHAndVMoves(r_or_q_idx)
            unsafe |= moves
            bQR &= ~r_or_q # remove current rook or queen
            r_or_q = (bQR & ~(bQR - 1))

        # king threats
        king = (bK & ~(bK - 1))
        king_span_c7_idx = 10
        while king != 0:
            king_idx = king.asBinaryString().index('1')
            # allign the king_span_c7 mask
            if king_idx <= king_span_c7_idx:
                moves = self.masks.king_span_c7 << (king_span_c7_idx - king_idx)
            else:
                moves = self.masks.king_span_c7 >> (king_idx - king_span_c7_idx)
            # remove moves sliding off board or allied pieces
            if king_idx % 8 < 4:
                moves &= ~self.masks.file_gh
            else:
                moves &= ~self.masks.file_ab
            unsafe |= moves
            bK &= ~king # remove current king
            king = (bK & ~(bK - 1))

        return unsafe
