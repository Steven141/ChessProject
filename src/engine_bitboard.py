"""
Bitboard Engine
"""

class GameState():
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
        # piece bitboards
        self.wP = self.wN = self.wB = self.wR = self.wQ = self.wK = 0
        self.bP = self.bN = self.bB = self.bR = self.bQ = self.bK = 0
        self.arrayToBitboard()

    
    def arrayToBitboard(self) -> None:
        for i in range(64):
            binary = '0000000000000000000000000000000000000000000000000000000000000000'
            binary = binary[:i] + '1' + binary[i+1:]
            match self.board[i // 8][i % 8]:
                case 'wP': self.wP += self.convertStringToBitboard(binary)
                case 'wN': self.wN += self.convertStringToBitboard(binary)
                case 'wB': self.wB += self.convertStringToBitboard(binary)
                case 'wR': self.wR += self.convertStringToBitboard(binary)
                case 'wQ': self.wQ += self.convertStringToBitboard(binary)
                case 'wK': self.wK += self.convertStringToBitboard(binary)
                case 'bP': self.bP += self.convertStringToBitboard(binary)
                case 'bN': self.bN += self.convertStringToBitboard(binary)
                case 'bB': self.bB += self.convertStringToBitboard(binary)
                case 'bR': self.bR += self.convertStringToBitboard(binary)
                case 'bQ': self.bQ += self.convertStringToBitboard(binary)
                case 'bK': self.bK += self.convertStringToBitboard(binary)


    def convertStringToBitboard(self, binary) -> int:
        int_rep = int(binary, 2)
        if binary[0] == '1': # negative, do 2's comp
            int_rep -= (1 << len(binary))
        return int_rep
    

    def drawBoard(self) -> None:
        new_board = [['--']*8 for _ in range(8)]
        for i in range(64):
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
        self.king_b7 = 0
        self.knight_c6 = 0
        self.not_white_pieces = 0 # all pieces white can capture (not black king)
        self.black_pieces = 0 # black pieces but no black king
        self.empty = 0


    def possibleMovesW(self, history, wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK) -> str:
        self.not_white_pieces = ~(wP|wN|wB|wR|wQ|wK|bK) # avoid illegal bK capture
        self.black_pieces = bP|bN|bB|bR|bQ # avoid illegal bK capture
        self.empty = ~(wP|wN|wB|wR|wQ|wK|bP|bN|bB|bR|bQ|bK)
        moves = self.possibleWP(history, wP)

        return moves


    def possibleWP(self, history, wP) -> str:
        moves = '' # x1,y1,x2,y2
        # continue at 5:30
        return moves


g = GameState()
g.drawBoard()
print(g.wP)
m = Moves()
m.possibleMovesW('', g.wP, g.wN, g.wB, g.wR, g.wQ, g.wK, g.bP, g.bN, g.bB, g.bR, g.bQ, g.bK)
