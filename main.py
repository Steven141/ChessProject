"""
File contains game driver logic.

Handles user input and displays current game state information.
"""


import pygame as pg
import engine, engine_advanced, ai_move_finder
from multiprocessing import Process, Queue


BOARD_WIDTH = BOARD_HEIGHT = 512
MOVE_LOG_PANEL_WIDTH = 250
MOVE_LOG_PANEL_HEIGHT = BOARD_HEIGHT
BOARD_DIM = 8
SQ_SIZE = BOARD_WIDTH // BOARD_DIM
FPS = 15
IMAGES = {}
PIECE_NAMES = ['bR', 'bN', 'bB', 'bQ', 'bK', 'bP', 'wR', 'wN', 'wB', 'wQ', 'wK', 'wP']


def loadImages() -> None:
    for piece in PIECE_NAMES:
        IMAGES[piece] = pg.transform.scale(
            pg.image.load(f'images/{piece}.png'),
            (SQ_SIZE, SQ_SIZE),
        )


def main() -> None:
    pg.init()
    screen = pg.display.set_mode((BOARD_WIDTH + MOVE_LOG_PANEL_WIDTH, BOARD_HEIGHT))
    clk = pg.time.Clock()
    screen.fill(pg.Color('white'))
    move_log_font = pg.font.SysFont('Arial', 14, False, False)
    game_state = engine_advanced.GameState()
    valid_moves = game_state.getValidMoves()
    move_made = False # flag for when move is made
    animate = False

    loadImages()
    running = True
    sq_selected: tuple[str] = () # last click of user (row, col)
    player_clicks: list[tuple[str]] = [] # keep track of selected squares
    game_over = False
    player_one = True # True if human is playing white. False if AI is playing
    player_two = True # True if human is playing black. False if AI is playing

    ai_thinking = False
    move_finder_process = None
    move_undone = False

    while running:
        is_human_turn = (game_state.whites_turn and player_one) or (not game_state.whites_turn and player_two)
        for event in pg.event.get():
            if event.type == pg.QUIT:
                running = False

            # mouse event cases
            elif event.type == pg.MOUSEBUTTONDOWN:
                if not game_over:
                    m_cord = pg.mouse.get_pos()
                    m_col, m_row = m_cord[0] // SQ_SIZE, m_cord[1] // SQ_SIZE
                    if sq_selected == (m_row, m_col) or m_col > 7: # same square clicked twice or user clicked mouse log
                        sq_selected = ()
                        player_clicks = []
                    else:
                        sq_selected = (m_row, m_col)
                        player_clicks.append(sq_selected)
                    if len(player_clicks) == 2 and is_human_turn:
                        move = engine_advanced.Move(player_clicks[0], player_clicks[1], game_state.board)
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
                if event.key == pg.K_z: # undo
                    game_state.undoMove()
                    move_made = True
                    animate = False
                    game_over = False
                    if ai_thinking:
                        move_finder_process.terminate()
                        ai_thinking = False
                    move_undone = True

                if event.key == pg.K_r: # reset
                    game_state = engine_advanced.GameState()
                    valid_moves = game_state.getValidMoves()
                    sq_selected = ()
                    player_clicks = []
                    move_made = False
                    animate = False
                    game_over = False
                    if ai_thinking:
                        move_finder_process.terminate()
                        ai_thinking = False

        # AI move finder
        if not game_over and not is_human_turn and not move_undone:
            if not ai_thinking:
                ai_thinking = True
                print('Thinking...')
                return_q = Queue() # used to pass data between threads
                move_finder_process = Process(target=ai_move_finder.findBestMove, args=(game_state, valid_moves, return_q))
                move_finder_process.start() # call findBestMove(game_state, valid_moves, return_q)

            if not move_finder_process.is_alive():
                print('Done thinking')
                ai_move = return_q.get()
                if not ai_move:
                    ai_move = ai_move_finder.findRandomMove(valid_moves)
                game_state.makeMove(ai_move)
                move_made = True
                animate = True
                ai_thinking = False

        if move_made:
            if animate:
                animateMove(game_state.move_log[-1], screen, game_state.board, clk)
            valid_moves = game_state.getValidMoves()
            move_made = False
            animate = False
            move_undone = False

        drawGameState(screen, game_state, valid_moves, sq_selected, move_log_font)

        if game_state.checkmate or game_state.stalemate:
            game_over = True
            drawEndGameText(screen, 'Stalemate' if game_state.stalemate else 'Black wins by checkmate' if game_state.whites_turn else 'White wins by checkmate')

        clk.tick(FPS)
        pg.display.flip()


"""
Responsible for all the graphics within a current game state
"""
def drawGameState(screen, game_state, valid_moves, sq_selected, move_log_font) -> None:
    drawBoard(screen)
    highlightSquares(screen, game_state, valid_moves, sq_selected)
    drawPieces(screen, game_state.board)
    drawMoveLog(screen, game_state, move_log_font)


"""
Draw the squares on the board
"""
def drawBoard(screen) -> None:
    global colors
    colors = (pg.Color('white'), pg.Color('gray'))
    for r in range(BOARD_DIM):
        for c in range(BOARD_DIM):
            color = colors[(r + c) % 2]
            pg.draw.rect(screen, color, pg.Rect(c*SQ_SIZE, r*SQ_SIZE, SQ_SIZE, SQ_SIZE))


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
Draw the pieces on the board
"""
def drawPieces(screen, board) -> None:
    for r in range(BOARD_DIM):
        for c in range(BOARD_DIM):
            piece = board[r][c]
            if piece != '--':
                screen.blit(IMAGES[piece], pg.Rect(c*SQ_SIZE, r*SQ_SIZE, SQ_SIZE, SQ_SIZE))


"""
Draw the move log
"""
def drawMoveLog(screen, game_state, font) -> None:
    move_log_rect = pg.Rect(BOARD_WIDTH, 0, MOVE_LOG_PANEL_WIDTH, MOVE_LOG_PANEL_HEIGHT)
    pg.draw.rect(screen, pg.Color('black'), move_log_rect)
    move_log = game_state.move_log
    move_texts = []
    for i in range(0, len(move_log), 2):
        move_str = f'{i//2 + 1}. {move_log[i]} '
        if i+1 < len(move_log):
            move_str += str(move_log[i+1]) + ' '
        move_texts.append(move_str)

    moves_per_row = 3
    padding = 5
    line_spacing = 2
    text_y = padding
    for i in range(0, len(move_texts), moves_per_row):
        text = ''
        for j in range(moves_per_row):
            if i + j < len(move_texts):
                text += move_texts[i+j]
        text_object = font.render(text, True, pg.Color('white'))
        text_location = move_log_rect.move(padding, text_y)
        screen.blit(text_object, text_location)
        text_y += text_object.get_height() + line_spacing


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
            if move.is_enpassant_move:
                enpassant_r = move.end_r + 1 if move.piece_captured[0] == 'b' else move.end_r - 1
                end_sq = pg.Rect(move.end_c*SQ_SIZE, enpassant_r*SQ_SIZE, SQ_SIZE, SQ_SIZE)
            screen.blit(IMAGES[move.piece_captured], end_sq)
        # draw moving piece
        screen.blit(IMAGES[move.piece_moved], pg.Rect(c*SQ_SIZE, r*SQ_SIZE, SQ_SIZE, SQ_SIZE))
        pg.display.flip()
        clk.tick(200)


def drawEndGameText(screen, text) -> None:
    font = pg.font.SysFont('Helvitca', 32, True, False)
    text_object = font.render(text, 0, pg.Color('Gray'))
    text_location = pg.Rect(0, 0, BOARD_WIDTH, BOARD_HEIGHT).move(BOARD_WIDTH/2 - text_object.get_width()/2, BOARD_HEIGHT/2 - text_object.get_height()/2)
    screen.blit(text_object, text_location)
    text_object = font.render(text, 0, pg.Color('Black'))
    screen.blit(text_object, text_location.move(2, 2))


if __name__ == "__main__":
    main()
