import ChessProject # rust library

result = ChessProject.sum_as_string(1, 2)
print(result)

a = ChessProject.GameState(8)
print(a)

c = ChessProject.CastleRights(True, True, True, False)
print(c)

m = ChessProject.Move((0,1), (2,3), is_castle_move=True)
print(m)
