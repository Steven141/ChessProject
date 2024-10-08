"""
File contains game driver logic using the rust engine library.

Handles user input and displays current game state information.
"""


import random
import pygame as pg
import argparse as argp
import ai_move_finder_rust
import ChessProject # rust engine library


BOARD_WIDTH = BOARD_HEIGHT = 512
MOVE_LOG_PANEL_WIDTH = 250
MOVE_LOG_PANEL_HEIGHT = BOARD_HEIGHT
BOARD_DIM = 8
SQ_SIZE = BOARD_WIDTH // BOARD_DIM
FPS = 15
IMAGES = {}
PIECE_NAMES = ['bR', 'bN', 'bB', 'bQ', 'bK', 'bP', 'wR', 'wN', 'wB', 'wQ', 'wK', 'wP']
DEPTH = 40


def loadImages() -> None:
    for piece in PIECE_NAMES:
        if piece[0] == 'w':
            key = piece[1]
        else:
            key = piece[1].lower()
        IMAGES[key] = pg.transform.scale(
            pg.image.load(f'images/{piece}.png'),
            (SQ_SIZE, SQ_SIZE),
        )


def main() -> None:
    parser = argp.ArgumentParser(description="Debug Mode")
    parser.add_argument(
        '-d',
        action='store_true',
        help='A boolean flag to activate some feature'
    )
    args = parser.parse_args()

    pg.init()
    screen = pg.display.set_mode((BOARD_WIDTH + MOVE_LOG_PANEL_WIDTH, BOARD_HEIGHT))
    clk = pg.time.Clock()
    screen.fill(pg.Color('white'))
    move_log_font = pg.font.SysFont('Arial', 14, False, False)
    file_row_char_font = pg.font.SysFont('Arial', 12, True, False)
    z = ChessProject.Zobrist()
    gs = ChessProject.GameState(z)
    bmf = ChessProject.BestMoveFinder(DEPTH)
    m = ChessProject.Moves()
    tt = ChessProject.TransTable()
    ob = ChessProject.OpeningBook()
    valid_moves = m.getValidMoves(z, gs.bitboards, gs.castle_rights, gs.hash_key, gs.whites_turn)
    move_made = False # flag for when move is made
    animate = False
    opening_node = ob.trie.root # keep track where we are in opeing book

    loadImages()
    running = True
    sq_selected: tuple[int] = () # last click of user (row, col)
    player_clicks: list[tuple[int]] = [] # keep track of selected squares
    game_over = False
    if args.d:
        player_one = True # True if human is playing white. False if AI is playing
        player_two = False # True if human is playing black. False if AI is playing
    else:
        while True:
            play_as_white = input("Would you like to play as white?\nEnter y or n: ")
            if play_as_white == 'y':
                player_one = True
                player_two = False
                break
            elif play_as_white == 'n':
                player_one = False
                player_two = True
                break

    ai_thinking = False
    move_undone = False

    while running:
        is_human_turn = (gs.whites_turn and player_one) or (not gs.whites_turn and player_two)
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
                        move = f'{player_clicks[0][0]}{player_clicks[0][1]}{player_clicks[1][0]}{player_clicks[1][1]}'
                        move_ep = f'{player_clicks[0][1]}{player_clicks[1][1]}wE' if gs.whites_turn else f'{player_clicks[0][1]}{player_clicks[1][1]}bE'
                        move_promo = f'{player_clicks[0][1]}{player_clicks[1][1]}QP' if gs.whites_turn else f'{player_clicks[0][1]}{player_clicks[1][1]}qP'
                        print(move)
                        for i in range(0, len(valid_moves), 4):
                            if move == valid_moves[i:i+4]:
                                gs.makeMove(m, z, valid_moves[i:i+4])
                                move_made = True
                                animate = True
                                sq_selected = ()
                                player_clicks = []
                                break
                            if move_ep == valid_moves[i:i+4]:
                                gs.makeMove(m, z, valid_moves[i:i+4])
                                move_made = True
                                animate = True
                                sq_selected = ()
                                player_clicks = []
                                break
                            if move_promo == valid_moves[i:i+4] and ((player_clicks[0][0] == 1 and player_clicks[1][0] == 0) if gs.whites_turn else (player_clicks[0][0] == 6 and player_clicks[1][0] == 7)):
                                gs.makeMove(m, z, valid_moves[i:i+4])
                                move_made = True
                                animate = True
                                sq_selected = ()
                                player_clicks = []
                                break
                        if not move_made:
                            player_clicks = [sq_selected]

        # AI move finder
        if not game_over and not is_human_turn and not move_undone:
            if not ai_thinking:
                ai_thinking = True
                ai_move = ''
                print('Thinking...\n')
                if gs.in_book_opening:
                    if len(gs.move_log) == 0: # ai makes first move
                        move, opening_node = random.choice(list(opening_node.children.items()))
                        ai_move = m.algebraToMove(move)
                        print(f"In Book Opening, Move: {move}\n")
                    else: # moves already made
                        if player_one or player_two: # human vs ai
                            last_move = m.moveToAlgebra(gs.move_log[-4:])
                            if last_move in opening_node.children and not opening_node.children[last_move].terminal: # prev move in opening book
                                opening_node = opening_node.children[last_move]
                                move, opening_node = random.choice(list(opening_node.children.items()))
                                ai_move = m.algebraToMove(move)
                                print(f"In Book Opening, Move: {move}\n")
                            else: # out of opeing book
                                gs.in_book_opening = False
                                ai_move = ai_move_finder_rust.findBestMove(gs, m, z, tt, bmf)
                        else: # ai vs ai
                            if not opening_node.terminal:
                                move, opening_node = random.choice(list(opening_node.children.items()))
                                ai_move = m.algebraToMove(move)
                                print(f"In Book Opening, Move: {move}\n")
                            else:
                                gs.in_book_opening = False
                                ai_move = ai_move_finder_rust.findBestMove(gs, m, z, tt, bmf)
                else:
                    ai_move = ai_move_finder_rust.findBestMove(gs, m, z, tt, bmf)
                print('Done thinking')
                if ai_move == '':
                    ai_move = ai_move_finder_rust.findRandomMove(valid_moves)
                gs.makeMove(m, z, ai_move)
                move_made = True
                animate = True
                ai_thinking = False

        if move_made:
            if animate:
                animateMove(gs.move_log[-4:], screen, gs, clk, file_row_char_font)
            valid_moves = m.getValidMoves(z, gs.bitboards, gs.castle_rights, gs.hash_key, gs.whites_turn)
            move_made = False
            animate = False
            move_undone = False

        drawGameState(screen, gs, valid_moves, sq_selected, move_log_font, file_row_char_font, m)

        if valid_moves == '':
            game_over = True
            drawEndGameText(screen, 'Stalemate' if m.stalemate else 'Black wins by checkmate' if gs.whites_turn else 'White wins by checkmate')

        clk.tick(FPS)
        pg.display.flip()


"""
Responsible for all the graphics within a current game state
"""
def drawGameState(screen, game_state, valid_moves, sq_selected, move_log_font, file_row_char_font, m) -> None:
    drawBoard(screen, file_row_char_font)
    highlightSquares(screen, game_state, valid_moves, sq_selected)
    drawPieces(screen, game_state.board)
    drawMoveLog(screen, game_state, move_log_font, m)


"""
Draw the squares on the board
"""
def drawBoard(screen, file_row_char_font) -> None:
    global colors
    colors = (pg.Color('white'), pg.Color('wheat4'))
    for r in range(BOARD_DIM):
        for c in range(BOARD_DIM):
            color = colors[(r + c) % 2]
            curr_rect = pg.Rect(c*SQ_SIZE, r*SQ_SIZE, SQ_SIZE, SQ_SIZE)
            pg.draw.rect(screen, color, curr_rect)
            drawFileRowChars(screen, curr_rect, file_row_char_font, r, c)


"""
Draw file and row chars on board
"""
def drawFileRowChars(screen, curr_rect, file_row_char_font, r, c) -> None:
    if c == 0:
        text_object = file_row_char_font.render(str(8-int(r)), True, colors[1] if r % 2 == 0 else colors[0])
        text_location = curr_rect.move(3, 1)
        screen.blit(text_object, text_location)
    if r == 7:
        text_object = file_row_char_font.render(chr(ord('a') + int(c)), True, colors[0] if c % 2 == 0 else colors[1])
        text_location = curr_rect.move(SQ_SIZE-7, SQ_SIZE-16)
        screen.blit(text_object, text_location)


"""
Highlight square selected and moves for piece selected
"""
def highlightSquares(screen, game_state, valid_moves, sq_selected) -> None:
    if sq_selected != ():
        r, c = sq_selected
        if game_state.board[r][c].isupper() if game_state.whites_turn else not game_state.board[r][c].isupper():
            # highlight selected square
            s = pg.Surface((SQ_SIZE, SQ_SIZE))
            s.set_alpha(100) # transperancy: 0 = transparent, 255 = opaque
            s.fill(pg.Color('red'))
            screen.blit(s, (c*SQ_SIZE, r*SQ_SIZE))
            # highlight moves from that square
            s.fill(pg.Color('yellow'))
            for i in range(0, len(valid_moves), 4):
                move = valid_moves[i:i+4]
                if move[3] == 'E' and (int(move[0]) == c and (r == 3 if game_state.whites_turn else r == 4)):
                    screen.blit(s, (int(move[1])*SQ_SIZE, (2 if game_state.whites_turn else 5)*SQ_SIZE))
                elif move[3] == 'P' and (int(move[0]) == c and (r == 1 if game_state.whites_turn else r == 6)):
                    screen.blit(s, (int(move[1])*SQ_SIZE, (0 if game_state.whites_turn else 7)*SQ_SIZE))
                elif int(move[0]) == r and int(move[1]) == c and move[3] != 'E' and move[3] != 'P':
                    screen.blit(s, (int(move[3])*SQ_SIZE, int(move[2])*SQ_SIZE))


"""
Draw the pieces on the board
"""
def drawPieces(screen, board, skip_sq=()) -> None:
    for r in range(BOARD_DIM):
        for c in range(BOARD_DIM):
            piece = board[r][c]
            if piece != ' ' and (r, c) != skip_sq:
                screen.blit(IMAGES[piece], pg.Rect(c*SQ_SIZE, r*SQ_SIZE, SQ_SIZE, SQ_SIZE))


"""
Draw the move log
"""
def drawMoveLog(screen, game_state, font, m) -> None:
    move_log_rect = pg.Rect(BOARD_WIDTH, 0, MOVE_LOG_PANEL_WIDTH, MOVE_LOG_PANEL_HEIGHT)
    pg.draw.rect(screen, pg.Color('black'), move_log_rect)
    move_log = game_state.move_log
    move_texts = [f"   {m.moveToAlgebra(move_log[i:i+4])}" for i in range(0, len(move_log), 4)]
    for i in range(0, len(move_texts), 2):
        move_count = i//2 + 1
        white_space = "             " if move_count % 2 == 0 else "    "
        move_texts[i] = f"{white_space}{i//2+1}.{move_texts[i]}"

    moves_per_row = 4
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
def animateMove(move, screen, gs, clk, file_row_char_font) -> None:
    global colors
    if move[3] == 'E' or move[3] == 'P':
        dR = -1 if not gs.whites_turn else 1
        dC = int(move[1]) - int(move[0])
    else:
        dR = int(move[2]) - int(move[0])
        dC = int(move[3]) - int(move[1])
    frames_per_sq = 10
    frame_count = (abs(dR) + abs(dC)) * frames_per_sq
    for f in range(frame_count + 1):
        if move[3] == 'E':
            r, c = (4 if gs.whites_turn else 3) + dR * f / frame_count, int(move[0]) + dC * f / frame_count
        elif move[3] == 'P':
            r, c = (6 if gs.whites_turn else 1) + dR * f / frame_count, int(move[0]) + dC * f / frame_count
        else:
            r, c = int(move[0]) + dR * f / frame_count, int(move[1]) + dC * f / frame_count
        drawBoard(screen, file_row_char_font)
        drawPieces(screen, gs.board, skip_sq=())
        # erase piece moved from its ending square
        if move[3] == 'P':
            end_r, end_c = 7 if gs.whites_turn else 0, int(move[1])
            color = colors[((7 if gs.whites_turn else 0) + int(move[1])) % 2]
            end_sq = pg.Rect(int(move[1])*SQ_SIZE, (7 if gs.whites_turn else 0)*SQ_SIZE, SQ_SIZE, SQ_SIZE)
        elif move[3] == 'E':
            end_r, end_c = 5 if gs.whites_turn else 2, int(move[1])
            color = colors[((5 if gs.whites_turn else 2) + int(move[1])) % 2]
            end_sq = pg.Rect(int(move[1])*SQ_SIZE, (5 if gs.whites_turn else 2)*SQ_SIZE, SQ_SIZE, SQ_SIZE)
        else:
            end_r, end_c = int(move[2]), int(move[3])
            color = colors[(int(move[2]) + int(move[3])) % 2]
            end_sq = pg.Rect(int(move[3])*SQ_SIZE, int(move[2])*SQ_SIZE, SQ_SIZE, SQ_SIZE)
        pg.draw.rect(screen, color, end_sq)
        drawFileRowChars(screen, end_sq, file_row_char_font, end_r, end_c)
        # draw captured piece onto rectange
        is_enpassant_move = (move[3] == 'E')
        if gs.recent_piece_captured != ' ':
            if is_enpassant_move:
                enpassant_r = (4 if gs.whites_turn else 3)
                end_sq = pg.Rect(int(move[1])*SQ_SIZE, enpassant_r*SQ_SIZE, SQ_SIZE, SQ_SIZE)
            screen.blit(IMAGES[gs.recent_piece_captured], end_sq)
        # draw moving piece
        screen.blit(IMAGES[gs.recent_piece_moved], pg.Rect(c*SQ_SIZE, r*SQ_SIZE, SQ_SIZE, SQ_SIZE))
        pg.display.flip()
        clk.tick(150)


def drawEndGameText(screen, text) -> None:
    font = pg.font.SysFont('Helvitca', 32, True, False)
    text_object = font.render(text, 0, pg.Color('Gray'))
    text_location = pg.Rect(0, 0, BOARD_WIDTH, BOARD_HEIGHT).move(BOARD_WIDTH/2 - text_object.get_width()/2, BOARD_HEIGHT/2 - text_object.get_height()/2)
    screen.blit(text_object, text_location)
    text_object = font.render(text, 0, pg.Color('Black'))
    screen.blit(text_object, text_location.move(2, 2))


if __name__ == "__main__":
    main()
