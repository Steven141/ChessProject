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
                    if move in valid_moves:
                        game_state.makeMove(move)
                        move_made = True
                    sq_selected = ()
                    player_clicks = []

            # key event cases
            elif event.type == pg.KEYDOWN:
                if event.key == pg.K_z:
                    game_state.undoMove()
                    move_made = True

        if move_made:
            valid_moves = game_state.getValidMoves()
            move_made = False

        drawGameState(screen, game_state)
        clk.tick(FPS)
        pg.display.flip()


def drawGameState(screen, game_state) -> None:
    drawBoard(screen)
    drawPieces(screen, game_state.board)


def drawBoard(screen) -> None:
    colors = (pg.Color('white'), pg.Color('gray'))
    for r in range(BOARD_DIM):
        for c in range(BOARD_DIM):
            color = colors[(r + c) % 2]
            pg.draw.rect(screen, color, pg.Rect(c*SQ_SIZE, r*SQ_SIZE, WIDTH, HEIGHT))


def drawPieces(screen, board) -> None:
    for r in range(BOARD_DIM):
        for c in range(BOARD_DIM):
            piece = board[r][c]
            if piece != '--':
                screen.blit(IMAGES[piece], pg.Rect(c*SQ_SIZE, r*SQ_SIZE, WIDTH, HEIGHT))


if __name__ == "__main__":
    main()
