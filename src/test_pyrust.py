import ChessProject # rust library
import time

gs = ChessProject.GameState()
gs.importFEN("rnbqk2r/ppp2pp1/3p1n1p/4p3/1bB1P3/1P3N1P/P1PP1PP1/RNBQ1RK1 b kq - 3 6")
m = ChessProject.Moves()
p = ChessProject.Perft(3)
bmf = ChessProject.BestMoveFinder(3)

start = time.time()
print(f'Execution Time = {time.time() - start}')
