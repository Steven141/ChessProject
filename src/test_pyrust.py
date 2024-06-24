import ChessProject # rust library
import time

gs = ChessProject.GameState()
gs.importFEN("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -")
m = ChessProject.Moves()
p = ChessProject.Perft(3)
start = time.time()
p.perftRoot(m, gs.wP, gs.wN, gs.wB, gs.wR, gs.wQ, gs.wK, gs.bP, gs.bN, gs.bB, gs.bR, gs.bQ, gs.bK, gs.EP, gs.cwK, gs.cwQ, gs.cbK, gs.cbQ, True, 0)
print(f'Total Moves = {p.total_move_counter}')
print(f'Execution Time = {time.time() - start}')
