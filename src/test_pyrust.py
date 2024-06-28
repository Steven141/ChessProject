import ChessProject # rust library
import time

gs = ChessProject.GameState()
gs.importFEN("rnbqk2r/ppp2pp1/3p1n1p/4p3/1bB1P3/1P3N1P/P1PP1PP1/RNBQ1RK1 b kq - 3 6")
m = ChessProject.Moves()
p = ChessProject.Perft(3)
bmf = ChessProject.BestMoveFinder(3)
x = bmf.negaMaxAlphaBeta(-5000, 5000, m, gs.wP, gs.wN, gs.wB, gs.wR, gs.wQ, gs.wK, gs.bP, gs.bN, gs.bB, gs.bR, gs.bQ, gs.bK, gs.EP, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, True, 0)
print(x)
print(bmf.considered_moves)
print(bmf.considered_moves[bmf.best_move_idx:(bmf.best_move_idx + 4)])
print(bmf.move_counter)
print(bmf.best_move_idx)
# start = time.time()
# p.perftRoot(m, gs.wP, gs.wN, gs.wB, gs.wR, gs.wQ, gs.wK, gs.bP, gs.bN, gs.bB, gs.bR, gs.bQ, gs.bK, gs.EP, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, True, 0)
# print(f'Total Moves = {p.total_move_counter}')
# print(f'Execution Time = {time.time() - start}')
