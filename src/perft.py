"""
Performance testing the bitboard engine.
"""


from engine_bitboard import Moves, GameState, BB_1
import time


class Perft():
    # static variables
    PERFT_MAX_DEPTH = 4
    perft_move_counter = 0
    perft_total_move_counter = 0


    """
    Convert r1c1r2c2 to rank file notation for moves
    """
    @staticmethod
    def moveToAlgebra(move) -> str:
        move_str = ''
        idx_to_file_ascii_shift = 49
        if move[3] == 'E': # enpassant
            move_str += chr(ord(move[0]) + idx_to_file_ascii_shift)
            move_str += '5' if move[2] == 'w' else '4'
            move_str += chr(ord(move[1]) + idx_to_file_ascii_shift)
            move_str += '6' if move[2] == 'w' else '3'
        elif move[3] == 'P': # pawn promo
            move_str += chr(ord(move[0]) + idx_to_file_ascii_shift)
            move_str += '7' if move[2].isupper() else '2'
            move_str += chr(ord(move[1]) + idx_to_file_ascii_shift)
            move_str += '8' if move[2].isupper() else '1'
            move_str += move[2]
        else: # regular move
            move_str += chr(ord(move[1]) + idx_to_file_ascii_shift)
            move_str += str(ord('8') - ord(move[0]))
            move_str += chr(ord(move[3]) + idx_to_file_ascii_shift)
            move_str += str(ord('8') - ord(move[2]))
        return move_str


    """
    Run the performace test
    """
    @classmethod
    def perft(cls, mm, wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK, EP, cwK, cwQ, cbK, cbQ, whites_turn, depth) -> None:
        if depth < cls.PERFT_MAX_DEPTH:
            if whites_turn:
                moves = mm.possibleMovesW(wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK, EP, cwK, cwQ, cbK, cbQ)
            else:
                moves = mm.possibleMovesB(wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK, EP, cwK, cwQ, cbK, cbQ)

            for i in range(0, len(moves), 4):
                wPt, wNt = mm.makeMove(wP, moves[i:i+4], 'P'), mm.makeMove(wN, moves[i:i+4], 'N')
                wBt, wRt = mm.makeMove(wB, moves[i:i+4], 'B'), mm.makeMove(wR, moves[i:i+4], 'R')
                wQt, wKt = mm.makeMove(wQ, moves[i:i+4], 'Q'), mm.makeMove(wK, moves[i:i+4], 'K')
                bPt, bNt = mm.makeMove(bP, moves[i:i+4], 'p'), mm.makeMove(bN, moves[i:i+4], 'n')
                bBt, bRt = mm.makeMove(bB, moves[i:i+4], 'b'), mm.makeMove(bR, moves[i:i+4], 'r')
                bQt, bKt = mm.makeMove(bQ, moves[i:i+4], 'q'), mm.makeMove(bK, moves[i:i+4], 'k')
                wRt, bRt = mm.makeMoveCastle(wRt, wK, moves[i:i+4], 'R'), mm.makeMoveCastle(bRt, bK, moves[i:i+4], 'r')
                EPt = mm.makeMoveEP(wP|bP, moves[i:i+4])

                cwKt, cwQt, cbKt, cbQt = cwK, cwQ, cbK, cbQ

                if moves[i + 3].isnumeric(): # regular move
                    start_shift = 64 - 1 - (int(moves[i]) * 8 + int(moves[i + 1]))
                    end_shift = 64 - 1 - (int(moves[i + 2]) * 8 + int(moves[i + 3]))
                    if ((BB_1 << start_shift) & wK) != 0: # white king move
                        cwKt = cwQt = False
                    if ((BB_1 << start_shift) & bK) != 0: # black king move
                        cbKt = cbQt = False
                    if ((BB_1 << start_shift) & wR & 1) != 0: # white king side rook move
                        cwKt = False
                    if ((BB_1 << start_shift) & wR & (BB_1 << 7)) != 0: # white queen side rook move
                        cwQt = False
                    if ((BB_1 << start_shift) & bR & (BB_1 << 56)) != 0: # black king side rook move
                        cbKt = False
                    if ((BB_1 << start_shift) & bR & (BB_1 << 63)) != 0: # black queen side rook move
                        cbQt = False
                    if ((BB_1 << end_shift) & 1) != 0: # white king side rook taken
                        cwKt = False
                    if ((BB_1 << end_shift) & (BB_1 << 7)) != 0: # white queen side rook taken
                        cwQt = False
                    if ((BB_1 << end_shift) & (BB_1 << 56)) != 0: # black king side rook taken
                        cbKt = False
                    if ((BB_1 << end_shift) & (BB_1 << 63)) != 0: # black queen side rook taken
                        cbQt = False

                if ((wKt & mm.unsafeForWhite(wPt, wNt, wBt, wRt, wQt, wKt, bPt, bNt, bBt, bRt, bQt, bKt)) == 0 and whites_turn) or ((bKt & mm.unsafeForBlack(wPt, wNt, wBt, wRt, wQt, wKt, bPt, bNt, bBt, bRt, bQt, bKt)) == 0 and not whites_turn):
                    if depth + 1 == cls.PERFT_MAX_DEPTH: # only count leaf nodes
                        cls.perft_move_counter += 1
                    # print(cls.perft_move_counter)
                    cls.perft(mm, wPt, wNt, wBt, wRt, wQt, wKt, bPt, bNt, bBt, bRt, bQt, bKt, EPt, cwKt, cwQt, cbKt, cbQt, not whites_turn, depth + 1)

        elif cls.perft_move_counter == 0:
            cls.perft_move_counter += 1


    """
    Run the performace test for each root node
    """
    @classmethod
    def perftRoot(cls, mm, wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK, EP, cwK, cwQ, cbK, cbQ, whites_turn, depth) -> None:
        if whites_turn:
            moves = mm.possibleMovesW(wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK, EP, cwK, cwQ, cbK, cbQ)
        else:
            moves = mm.possibleMovesB(wP, wN, wB, wR, wQ, wK, bP, bN, bB, bR, bQ, bK, EP, cwK, cwQ, cbK, cbQ)

        for i in range(0, len(moves), 4):
            wPt, wNt = mm.makeMove(wP, moves[i:i+4], 'P'), mm.makeMove(wN, moves[i:i+4], 'N')
            wBt, wRt = mm.makeMove(wB, moves[i:i+4], 'B'), mm.makeMove(wR, moves[i:i+4], 'R')
            wQt, wKt = mm.makeMove(wQ, moves[i:i+4], 'Q'), mm.makeMove(wK, moves[i:i+4], 'K')
            bPt, bNt = mm.makeMove(bP, moves[i:i+4], 'p'), mm.makeMove(bN, moves[i:i+4], 'n')
            bBt, bRt = mm.makeMove(bB, moves[i:i+4], 'b'), mm.makeMove(bR, moves[i:i+4], 'r')
            bQt, bKt = mm.makeMove(bQ, moves[i:i+4], 'q'), mm.makeMove(bK, moves[i:i+4], 'k')
            wRt, bRt = mm.makeMoveCastle(wRt, wK, moves[i:i+4], 'R'), mm.makeMoveCastle(bRt, bK, moves[i:i+4], 'r')
            EPt = mm.makeMoveEP(wP|bP, moves[i:i+4])

            cwKt, cwQt, cbKt, cbQt = cwK, cwQ, cbK, cbQ

            if moves[i + 3].isnumeric(): # regular move
                start_shift = 64 - 1 - (int(moves[i]) * 8 + int(moves[i + 1]))
                end_shift = 64 - 1 - (int(moves[i + 2]) * 8 + int(moves[i + 3]))
                if ((BB_1 << start_shift) & wK) != 0: # white king move
                    cwKt = cwQt = False
                if ((BB_1 << start_shift) & bK) != 0: # black king move
                    cbKt = cbQt = False
                if ((BB_1 << start_shift) & wR & 1) != 0: # white king side rook move
                    cwKt = False
                if ((BB_1 << start_shift) & wR & (BB_1 << 7)) != 0: # white queen side rook move
                    cwQt = False
                if ((BB_1 << start_shift) & bR & (BB_1 << 56)) != 0: # black king side rook move
                    cbKt = False
                if ((BB_1 << start_shift) & bR & (BB_1 << 63)) != 0: # black queen side rook move
                    cbQt = False
                if ((BB_1 << end_shift) & 1) != 0: # white king side rook taken
                    cwKt = False
                if ((BB_1 << end_shift) & (BB_1 << 7)) != 0: # white queen side rook taken
                    cwQt = False
                if ((BB_1 << end_shift) & (BB_1 << 56)) != 0: # black king side rook taken
                    cbKt = False
                if ((BB_1 << end_shift) & (BB_1 << 63)) != 0: # black queen side rook taken
                    cbQt = False

            if ((wKt & mm.unsafeForWhite(wPt, wNt, wBt, wRt, wQt, wKt, bPt, bNt, bBt, bRt, bQt, bKt)) == 0 and whites_turn) or ((bKt & mm.unsafeForBlack(wPt, wNt, wBt, wRt, wQt, wKt, bPt, bNt, bBt, bRt, bQt, bKt)) == 0 and not whites_turn):
                cls.perft(mm, wPt, wNt, wBt, wRt, wQt, wKt, bPt, bNt, bBt, bRt, bQt, bKt, EPt, cwKt, cwQt, cbKt, cbQt, not whites_turn, depth + 1)
                print(f'{cls.moveToAlgebra(moves[i:i+4])} {cls.perft_move_counter}')
                cls.perft_total_move_counter += cls.perft_move_counter
                cls.perft_move_counter = 0


gs = GameState()
# gs.importFEN('3k4/8/8/8/8/8/8/R3K3 w Q - 0 1')

gs.drawGameArray()

mm = Moves()
start = time.time()
Perft.perftRoot(mm, gs.wP, gs.wN, gs.wB, gs.wR, gs.wQ, gs.wK, gs.bP, gs.bN, gs.bB, gs.bR, gs.bQ, gs.bK, gs.EP, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, True, 0)

print(f'Total Moves = {Perft.perft_total_move_counter}')
print(f'Execution Time = {time.time() - start}')
