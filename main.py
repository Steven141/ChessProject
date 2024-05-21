"""
File contains game driver logic.

Handles user input and displays current game state information.
"""


import pygame as pg
import engine


WIDTH = HEIGHT = 512
BOARD_DIM = 8
SQ_SIZE = WIDTH // BOARD_DIM
FPS = 15
IMAGES = {}
PIECE_NAMES = [
    'bR', 'bN', 'bB', 'bQ', 'bK', 'bP',
    'wR', 'wN', 'wB', 'wQ', 'wK', 'wP',
]


def loadImages() -> None:
    for piece in PIECE_NAMES:
        IMAGES[piece] = pg.transform.scale(
            pg.image.load(f'images/{piece}.png'),
            (SQ_SIZE, SQ_SIZE),
        )


def main() -> None:
    pg.init()
    screen = pg.display.set_mode((WIDTH, HEIGHT))
    clk = pg.time.Clock()
    screen.fill(pg.Color('white'))
    game_state = engine.GameState()
    valid_moves = game_state.getValidMoves()
    move_made = False # flag for when move is made
    animate = False

    loadImages()
    running = True
    sq_selected: tuple[str] = () # last click of user (row, col)
    player_clicks: list[tuple[str]] = [] # keep track of selected squares

    while running:
        for event in pg.event.get():
            if event.type == pg.QUIT:
                running = False

            # mouse event cases
            elif event.type == pg.MOUSEBUTTONDOWN:
                m_cord = pg.mouse.get_pos()
                m_col, m_row = m_cord[0] // SQ_SIZE, m_cord[1] // SQ_SIZE
                if sq_selected == (m_row, m_col): # same square clicked twice
                    sq_selected = ()
                    player_clicks = []
                else:
                    sq_selected = (m_row, m_col)
                    player_clicks.append(sq_selected)
                if len(player_clicks) == 2:
                    move = engine.Move(player_clicks[0], player_clicks[1], game_state.board)
                    print(move.getChessNotation())
                    for i in range(len(valid_moves)):
                        if move == valid_moves[i]:
                            game_state.makeMove(valid_moves[i])
                            move_made = True
                            animate = True
                            sq_selected = ()
                            player_clicks = []
                    if not move_made:
                        player_clicks = [sq_selected]

            # key event cases
            elif event.type == pg.KEYDOWN:
                if event.key == pg.K_z:
                    game_state.undoMove()
                    move_made = True
                    animate = False

        if move_made:
            if animate:
                animateMove(game_state.move_log[-1], screen, game_state.board, clk)
            valid_moves = game_state.getValidMoves()
            move_made = False
            animate = False

        drawGameState(screen, game_state, valid_moves, sq_selected)
        clk.tick(FPS)
        pg.display.flip()


"""
Highlight square selected and moves for piece selected
"""
def highlightSquares(screen, game_state, valid_moves, sq_selected) -> None:
    if sq_selected != ():
        r, c = sq_selected
        if game_state.board[r][c][0] == ('w' if game_state.whites_turn else 'b'):
            # highlight selected square
            s = pg.Surface((SQ_SIZE, SQ_SIZE))
            s.set_alpha(100) # transperancy: 0 = transparent, 255 = opaque
            s.fill(pg.Color('blue'))
            screen.blit(s, (c*SQ_SIZE, r*SQ_SIZE))
            # highlight moves from that square
            s.fill(pg.Color('yellow'))
            for move in valid_moves:
                if move.start_r == r and move.start_c == c:
                    screen.blit(s, (move.end_c*SQ_SIZE, move.end_r*SQ_SIZE))


"""
Responsible for all the graphics within a current game state
"""
def drawGameState(screen, game_state, valid_moves, sq_selected) -> None:
    drawBoard(screen)
    highlightSquares(screen, game_state, valid_moves, sq_selected)
    drawPieces(screen, game_state.board)


"""
Draw the squares on the board
"""
def drawBoard(screen) -> None:
    global colors
    colors = (pg.Color('white'), pg.Color('gray'))
    for r in range(BOARD_DIM):
        for c in range(BOARD_DIM):
            color = colors[(r + c) % 2]
            pg.draw.rect(screen, color, pg.Rect(c*SQ_SIZE, r*SQ_SIZE, WIDTH, HEIGHT))


"""
Draw the pieces on the board
"""
def drawPieces(screen, board) -> None:
    for r in range(BOARD_DIM):
        for c in range(BOARD_DIM):
            piece = board[r][c]
            if piece != '--':
                screen.blit(IMAGES[piece], pg.Rect(c*SQ_SIZE, r*SQ_SIZE, SQ_SIZE, SQ_SIZE))


"""
Animating a move
"""
def animateMove(move, screen, board, clk) -> None:
    global colors
    dR = move.end_r - move.start_r
    dC = move.end_c - move.start_c
    frames_per_sq = 10
    frame_count = (abs(dR) + abs(dC)) * frames_per_sq
    for f in range(frame_count + 1):
        r, c = move.start_r + dR * f / frame_count, move.start_c + dC * f / frame_count
        drawBoard(screen)
        drawPieces(screen, board)
        # erase piece moved from its ending square
        color = colors[(move.end_r + move.end_c) % 2]
        end_sq = pg.Rect(move.end_c*SQ_SIZE, move.end_r*SQ_SIZE, SQ_SIZE, SQ_SIZE)
        pg.draw.rect(screen, color, end_sq)
        # draw captured piece onto rectange
        if move.piece_captured != '--':
            screen.blit(IMAGES[move.piece_captured], end_sq)
        # draw moving piece
        screen.blit(IMAGES[move.piece_moved], pg.Rect(c*SQ_SIZE, r*SQ_SIZE, SQ_SIZE, SQ_SIZE))
        pg.display.flip()
        clk.tick(60)


if __name__ == "__main__":
    main()
