"""
Bitboard Engine
"""

class GameState():
    """
    In this 8x8 game board, the first char represents color
    and the second char represents type of piece. Empty
    square is denoted with '--'.
    """
    def __init__(self) -> None:
        # self.board = [
        #     ['bR', 'bN', 'bB', 'bQ', 'bK', 'bB', 'bN', 'bR'],
        #     ['bP', 'bP', 'bP', 'bP', 'bP', 'bP', 'bP', 'bP'],
        #     ['--', '--', '--', '--', '--', '--', '--', '--'],
        #     ['--', '--', '--', '--', '--', '--', '--', '--'],
        #     ['--', '--', '--', '--', '--', '--', '--', '--'],
        #     ['--', '--', '--', '--', '--', '--', '--', '--'],
        #     ['wP', 'wP', 'wP', 'wP', 'wP', 'wP', 'wP', 'wP'],
        #     ['wR', 'wN', 'wB', 'wQ', 'wK', 'wB', 'wN', 'wR'],
        # ]
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
        # piece bitboards
        self.wP = self.wN = self.wB = self.wR = self.wQ = self.wK = 0
        self.bP = self.bN = self.bB = self.bR = self.bQ = self.bK = 0
        self.arrayToBitboard()


    """
    Populate piece bitboards from array representation of game board
    """
    def arrayToBitboard(self) -> None:
        for i in range(64):
            binary = '0000000000000000000000000000000000000000000000000000000000000000'
            binary = binary[:i] + '1' + binary[i+1:]
            match self.board[i // 8][i % 8]:
                case 'wP': self.wP += BinaryOps.convertStringToBitboard(binary)
                case 'wN': self.wN += BinaryOps.convertStringToBitboard(binary)
                case 'wB': self.wB += BinaryOps.convertStringToBitboard(binary)
                case 'wR': self.wR += BinaryOps.convertStringToBitboard(binary)
                case 'wQ': self.wQ += BinaryOps.convertStringToBitboard(binary)
                case 'wK': self.wK += BinaryOps.convertStringToBitboard(binary)
                case 'bP': self.bP += BinaryOps.convertStringToBitboard(binary)
                case 'bN': self.bN += BinaryOps.convertStringToBitboard(binary)
                case 'bB': self.bB += BinaryOps.convertStringToBitboard(binary)
                case 'bR': self.bR += BinaryOps.convertStringToBitboard(binary)
                case 'bQ': self.bQ += BinaryOps.convertStringToBitboard(binary)
                case 'bK': self.bK += BinaryOps.convertStringToBitboard(binary)


    """
    Prints the current state of the game
    """
    def drawGameArray(self) -> None:
        new_board = [['--']*8 for _ in range(8)]
        for i in range(64): # i = 0 -> board[0][0] -> bitboard_as_bin[0]
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


class Moves():
    def __init__(self) -> None:
        # specific bitboards
        self.file_a = -9187201950435737472
        self.file_h = 72340172838076673
        self.file_ab = -4557430888798830400
        self.file_gh = 217020518514230019
        self.rank_1 = 255
        self.rank_4 = 4278190080
        self.rank_5 = 1095216660480
        self.rank_8 = -72057594037927936
        self.centre = 103481868288
        self.extended_centre = 66229406269440
        self.king_side = 1085102592571150095
        self.queen_side = -1085102592571150096
        self.king_span_c7 = 8093091675687092224 # where c7 king can attack
        self.knight_span_c6 = 5802888705324613632 # where c6 knight can attack
        self.not_white_pieces = 0 # all pieces white can capture (not black king)
        self.black_pieces = 0 # black pieces but no black king
        self.empty = 0
        self.occupied = 0

        # region based bitboard masks
        self.rank_masks = [
            self.rank_8,
            71776119061217280,
            280375465082880,
            self.rank_5,
            self.rank_4,
            16711680,
            65280,
            self.rank_1,
        ] # from rank 8 to rank 1
        self.file_masks = [
            self.file_a,
            4629771061636907072,
            2314885530818453536,
            1157442765409226768,
            578721382704613384,
            289360691352306692,
            144680345676153346,
            self.file_h,
        ] # from file a to file h
        self.diagonal_masks = [
            -9223372036854775808,
            4647714815446351872,
            2323998145211531264,
            1161999622361579520,
            580999813328273408,
            290499906672525312,
            145249953336295424,
            72624976668147840,
            283691315109952,
            1108169199648,
            4328785936,
            16909320,
            66052,
            258,
            1,
        ] # from top left to bottom right
        self.anti_diagonal_masks = [
            72057594037927936,
            144396663052566528,
            288794425616760832,
            577588855528488960,
            1155177711073755136,
            2310355422147575808,
            4620710844295151872,
            -9205322385119247871,
            36099303471055874,
            141012904183812,
            550831656968,
            2151686160,
            8405024,
            32832,
            128,
        ] # from top right to bottom left


    """
    Return a move list string containing all possible moves for white
    """
    def possibleMovesW(self, history, wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK) -> str:
        self.not_white_pieces = ~(wP|wN|wB|wR|wQ|wK|bK) # avoid illegal bK capture
        self.black_pieces = bP|bN|bB|bR|bQ # avoid illegal bK capture
        self.empty = ~(wP|wN|wB|wR|wQ|wK|bP|bN|bB|bR|bQ|bK)
        self.occupied = ~self.empty
        move_list = self.possibleWP(history, wP, bP) + self.possibleWB(wB) + self.possibleWQ(wQ) + self.possibleWR(wR) + self.possibleWN(wN) + self.possibleWK(wK)

        return move_list


    """
    Return a move list string containing all possible moves for a white pawn
    """
    def possibleWP(self, history, wP, bP) -> str:
        """
        TODO: look into faster method - tut 6
        moves = (wP << 7) & self.black_pieces & ~self.rank_8 & ~self.file_a # right capture
        possible_move = moves & ~(moves - 1) # selects single possible move
        while possible_move != 0:
            possible_move_bin = BinaryOps.convertBitboardToString(possible_move)
            idx = possible_move_bin.index('1')
            move_list += f'{(idx // 8) + 1}{(idx % 8) - 1}{idx // 8}{idx % 8}'
            moves &= ~possible_move
            possible_move = moves & ~(moves - 1)
        """
        # standard moves and captures
        move_list = '' # r1,c1,r2,c2
        moves = (wP << 7) & self.black_pieces & ~self.rank_8 & ~self.file_a # right capture
        for i in range(64): # i = 0 -> board[0][0] -> bitboard_as_bin[0]
            shift = 64 - 1 - i
            if (moves >> shift) & 1 == 1:
                move_list += f'{(i // 8) + 1}{(i % 8) - 1}{i // 8}{i % 8}'

        moves = (wP << 9) & self.black_pieces & ~self.rank_8 & ~self.file_h # left capture
        for i in range(64):
            shift = 64 - 1 - i
            if (moves >> shift) & 1 == 1:
                move_list += f'{(i // 8) + 1}{(i % 8) + 1}{i // 8}{i % 8}'

        moves = (wP << 8) & self.empty & ~self.rank_8 # move forward 1
        for i in range(64):
            shift = 64 - 1 - i
            if (moves >> shift) & 1 == 1:
                move_list += f'{(i // 8) + 1}{i % 8}{i // 8}{i % 8}'

        moves = (wP << 16) & self.empty & (self.empty << 8) & self.rank_4 # move forward 2
        for i in range(64):
            shift = 64 - 1 - i
            if (moves >> shift) & 1 == 1:
                move_list += f'{(i // 8) + 2}{i % 8}{i // 8}{i % 8}'

        # pawn promotion, move_list -> c1,c2,promo type,'P'
        moves = (wP << 7) & self.black_pieces & self.rank_8 & ~self.file_a # promo by right capture
        for i in range(64):
            shift = 64 - 1 - i
            if (moves >> shift) & 1 == 1:
                c1, c2 = (i % 8) - 1, i % 8
                move_list += f'{c1}{c2}QP{c1}{c2}RP{c1}{c2}BP{c1}{c2}NP'

        moves = (wP << 9) & self.black_pieces & self.rank_8 & ~self.file_h # promo by left capture
        for i in range(64):
            shift = 64 - 1 - i
            if (moves >> shift) & 1 == 1:
                c1, c2 = (i % 8) + 1, i % 8
                move_list += f'{c1}{c2}QP{c1}{c2}RP{c1}{c2}BP{c1}{c2}NP'

        moves = (wP << 8) & self.empty & self.rank_8 # promo by move forward 1
        for i in range(64):
            shift = 64 - 1 - i
            if (moves >> shift) & 1 == 1:
                c1 = c2 = i % 8
                move_list += f'{c1}{c2}QP{c1}{c2}RP{c1}{c2}BP{c1}{c2}NP'

        # enpassant, move_list -> c1,c2,space,'E'
        if len(history) >= 4:
            if (history[-1] == history[-3]) and (abs(int(history[-2]) - int(history[-4])) == 2):
                file = int(history[-1])
                moves = (wP >> 1) & bP & self.rank_5 & ~self.file_a & self.file_masks[file] # enpassant right
                for i in range(64):
                    shift = 64 - 1 - i
                    if (moves >> shift) & 1 == 1:
                        c1, c2 = (i % 8) - 1, i % 8
                        move_list += f'{c1}{c2} E'

                moves = (wP << 1) & bP & self.rank_5 & ~self.file_h & self.file_masks[file] # enpassant left
                for i in range(64):
                    shift = 64 - 1 - i
                    if (moves >> shift) & 1 == 1:
                        c1, c2 = (i % 8) + 1, i % 8
                        move_list += f'{c1}{c2} E'

        return move_list


    """
    Return a move list string containing all possible moves for a white bishop
    """
    def possibleWB(self, wB) -> str:
        move_list = ''
        bishop = wB & ~(wB - 1)

        while bishop != 0:
            bishop_idx = BinaryOps.convertBitboardToString(bishop).index('1')
            moves = self.possibleDiagAndAntiDiagMoves(bishop_idx) & self.not_white_pieces
            possible_move = moves & ~(moves - 1) # selects single possible move

            while possible_move != 0:
                move_idx = BinaryOps.convertBitboardToString(possible_move).index('1')
                move_list += f'{bishop_idx // 8}{bishop_idx % 8}{move_idx // 8}{move_idx % 8}'
                moves &= ~possible_move # remove current possible move
                possible_move = moves & ~(moves - 1)

            wB &= ~bishop # remove current bishop
            bishop = wB & ~(wB - 1)

        return move_list


    """
    Return a move list string containing all possible moves for a white queen
    """
    def possibleWQ(self, wQ) -> str:
        move_list = ''
        queen = wQ & ~(wQ - 1)

        while queen != 0:
            queen_idx = BinaryOps.convertBitboardToString(queen).index('1')
            moves = (self.possibleDiagAndAntiDiagMoves(queen_idx) | self.possibleHAndVMoves(queen_idx)) & self.not_white_pieces
            possible_move = moves & ~(moves - 1) # selects single possible move

            while possible_move != 0:
                move_idx = BinaryOps.convertBitboardToString(possible_move).index('1')
                move_list += f'{queen_idx // 8}{queen_idx % 8}{move_idx // 8}{move_idx % 8}'
                moves &= ~possible_move # remove current possible move
                possible_move = moves & ~(moves - 1)

            wQ &= ~queen # remove current queen
            queen = wQ & ~(wQ - 1)

        return move_list


    """
    Return a move list string containing all possible moves for a white rook
    """
    def possibleWR(self, wR) -> str:
        move_list = ''
        rook = wR & ~(wR - 1)

        while rook != 0:
            rook_idx = BinaryOps.convertBitboardToString(rook).index('1')
            moves = self.possibleHAndVMoves(rook_idx) & self.not_white_pieces
            possible_move = moves & ~(moves - 1) # selects single possible move

            while possible_move != 0:
                move_idx = BinaryOps.convertBitboardToString(possible_move).index('1')
                move_list += f'{rook_idx // 8}{rook_idx % 8}{move_idx // 8}{move_idx % 8}'
                moves &= ~possible_move # remove current possible move
                possible_move = moves & ~(moves - 1)

            wR &= ~rook # remove current rook
            rook = wR & ~(wR - 1)

        return move_list


    """
    Return a move list string containing all possible moves for a white knight
    """
    def possibleWN(self, wN) -> str:
        move_list = ''
        knight = wN & ~(wN - 1)
        knight_span_c6_idx = 18

        while knight != 0:
            knight_idx = BinaryOps.convertBitboardToString(knight).index('1')

            # allign the knight_span_c6 mask
            if knight_idx <= knight_span_c6_idx:
                moves = self.knight_span_c6 << (knight_span_c6_idx - knight_idx)
            else:
                moves = self.knight_span_c6 >> (knight_idx - knight_span_c6_idx)

            # remove moves sliding off board or allied pieces
            if knight_idx % 8 < 4:
                moves &= (~self.file_gh) & self.not_white_pieces
            else:
                moves &= (~self.file_ab) & self.not_white_pieces
            possible_move = moves & ~(moves - 1) # selects single possible move

            while possible_move != 0:
                move_idx = BinaryOps.convertBitboardToString(possible_move).index('1')
                move_list += f'{knight_idx // 8}{knight_idx % 8}{move_idx // 8}{move_idx % 8}'
                moves &= ~possible_move # remove current possible move
                possible_move = moves & ~(moves - 1)

            wN &= ~knight # remove current knight
            knight = wN & ~(wN - 1)

        return move_list


    """
    Return a move list string containing all possible moves for a white king
    """
    def possibleWK(self, wK) -> str:
        move_list = ''
        king = wK & ~(wK - 1)
        king_span_c7_idx = 10

        while king != 0:
            king_idx = BinaryOps.convertBitboardToString(king).index('1')

            # allign the king_span_c7 mask
            if king_idx <= king_span_c7_idx:
                moves = self.king_span_c7 << (king_span_c7_idx - king_idx)
            else:
                moves = self.king_span_c7 >> (king_idx - king_span_c7_idx)

            # remove moves sliding off board or allied pieces
            if king_idx % 8 < 4:
                moves &= (~self.file_gh) & self.not_white_pieces
            else:
                moves &= (~self.file_ab) & self.not_white_pieces
            possible_move = moves & ~(moves - 1) # selects single possible move

            while possible_move != 0:
                move_idx = BinaryOps.convertBitboardToString(possible_move).index('1')
                move_list += f'{king_idx // 8}{king_idx % 8}{move_idx // 8}{move_idx % 8}'
                moves &= ~possible_move # remove current possible move
                possible_move = moves & ~(moves - 1)

            wK &= ~king # remove current king
            king = wK & ~(wK - 1)

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
        binary_idx = 1 << (64 - 1 - piece_idx)
        rank_mask = self.rank_masks[piece_idx // 8]
        file_mask = self.file_masks[piece_idx % 8]

        possible_h = (self.occupied - 2*binary_idx) ^ BinaryOps.reverseBits(BinaryOps.reverseBits(self.occupied) - 2*BinaryOps.reverseBits(binary_idx))
        possible_v = ((self.occupied & file_mask) - 2*binary_idx) ^ BinaryOps.reverseBits(BinaryOps.reverseBits(self.occupied & file_mask) - 2*BinaryOps.reverseBits(binary_idx))

        return (possible_h & rank_mask) | (possible_v & file_mask)


    """
    Returns all possible diagonal and anti-diagonal moves of piece at index piece_idx

    See possibleHAndVMoves func description for formula derivation
    """
    def possibleDiagAndAntiDiagMoves(self, piece_idx) -> int:
        # piece_idx = 0 -> top left of board -> 1000...000
        binary_idx = 1 << (64 - 1 - piece_idx)
        diag_mask = self.diagonal_masks[(piece_idx // 8) + (piece_idx % 8)]
        a_diag_mask = self.anti_diagonal_masks[(piece_idx // 8) - (piece_idx % 8) + 7]

        possible_d = ((self.occupied & diag_mask) - 2*binary_idx) ^ BinaryOps.reverseBits(BinaryOps.reverseBits((self.occupied & diag_mask)) - 2*BinaryOps.reverseBits(binary_idx))
        possible_ad = ((self.occupied & a_diag_mask) - 2*binary_idx) ^ BinaryOps.reverseBits(BinaryOps.reverseBits(self.occupied & a_diag_mask) - 2*BinaryOps.reverseBits(binary_idx))

        return (possible_d & diag_mask) | (possible_ad & a_diag_mask)


class BinaryOps():
    """
    Converts a binary string to a bitboard
    """
    @staticmethod
    def convertStringToBitboard(binary) -> int:
        int_rep = int(binary, 2)
        if binary[0] == '1': # negative, do 2's comp
            int_rep -= (1 << len(binary))
        return int_rep


    """
    Prints a bitboard in array form
    """
    @staticmethod
    def drawArrayFromBitboard(bitboard) -> None:
        new_board = [['0']*8 for _ in range(8)]
        for i in range(64):
            shift = 64 - 1 - i
            if (bitboard >> shift) & 1 == 1:
                new_board[i // 8][i % 8] = '1'
        for r in new_board:
            print(*r)


    """
    Takes an int as input and returns an int representing the reverse of the inputed binary
    """
    @staticmethod
    def reverseBits(int64) -> int:
        bin_str = BinaryOps.convertBitboardToString(int64)
        return BinaryOps.convertStringToBitboard(bin_str[::-1])


    """
    Takes an int as input and returns its binary string

    Special Cases:
    1. append a postfix of 1 for negative numbers
    2. bin() does not keep leading zeros to use f'{int64:064b}'
        - int64 -> int to convert to binary string
        - : -> everything after this is the format specifier
        - 0 -> pad with zeros
        - 64 -> pad to a total length off 64
        - b -> use binary representation for the number
    """
    @staticmethod
    def convertBitboardToString(int64) -> str:
        consider_window = int('1'*64, 2)
        return f'{int64 & consider_window :064b}'


g = GameState()
g.drawGameArray()

m = Moves()
move_list = m.possibleMovesW('', g.wP, g.wN, g.wB, g.wR, g.wQ, g.wK, g.bP, g.bN, g.bB, g.bR, g.bQ, g.bK)
