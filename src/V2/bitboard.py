"""
Define what a bitboard is and functions to operate on them.

Contains useful bitboard masks
"""


import numpy as np


class BitBoard():
    def __init__(self, int64=0, bin_str='') -> None:
        if bin_str != '':
            if len(bin_str) == 64:
                int_repr = int(bin_str, 2)
                if bin_str[0] == '1': # negative, do 2's comp
                    int_repr -= (1 << 64)
                self.int64 = np.int64(int_repr)
            else:
                raise ValueError('Binary string must contain 64 bits')
        else:
            self.int64 = np.int64(int64)


    """
    Returns a new BitBoard representing the reverse of the original BitBoard binary
    """
    def reverseBits(self) -> 'BitBoard':
        return BitBoard(bin_str=self.asBinaryString()[::-1])


    def asBinaryString(self) -> str:
        return np.binary_repr(self.int64, width=64)


    """
    Prints a bitboard in array form
    """
    def drawArray(self) -> None:
        new_board = [['0']*8 for _ in range(8)]
        for i in range(64):
            shift = 64 - 1 - i
            if (self >> shift) & 1 == 1:
                new_board[i // 8][i % 8] = '1'
        for r in new_board:
            print(*r)


    def __eq__(self, obj) -> bool:
        if isinstance(obj, int):
            return self.int64 == obj
        elif isinstance(obj, BitBoard):
            return self.int64 == obj.int64
        else:
            raise ValueError('Must be an instance of BitBoard class or int')


    def __lt__(self, obj) -> bool:
        if isinstance(obj, int):
            return self.int64 < obj
        elif isinstance(obj, BitBoard):
            return self.int64 < obj.int64
        else:
            raise ValueError('Must be an instance of BitBoard class or int')


    def __gt__(self, obj) -> bool:
        if isinstance(obj, int):
            return self.int64 > obj
        elif isinstance(obj, BitBoard):
            return self.int64 > obj.int64
        else:
            raise ValueError('Must be an instance of BitBoard class or int')


    def __le__(self, obj) -> bool:
        if isinstance(obj, int):
            return self.int64 <= obj
        elif isinstance(obj, BitBoard):
            return self.int64 <= obj.int64
        else:
            raise ValueError('Must be an instance of BitBoard class or int')


    def __ge__(self, obj) -> bool:
        if isinstance(obj, int):
            return self.int64 >= obj
        elif isinstance(obj, BitBoard):
            return self.int64 >= obj.int64
        else:
            raise ValueError('Must be an instance of BitBoard class or int')


    def __add__(self, obj) -> 'BitBoard':
        if isinstance(obj, int):
            return BitBoard(self.int64 + obj)
        elif isinstance(obj, BitBoard):
            return BitBoard(self.int64 + obj.int64)
        else:
            raise ValueError('Must be an instance of BitBoard class or int')


    def __sub__(self, obj) -> 'BitBoard':
        if isinstance(obj, int):
            return BitBoard(self.int64 - obj)
        elif isinstance(obj, BitBoard):
            return BitBoard(self.int64 - obj.int64)
        else:
            raise ValueError('Must be an instance of BitBoard class or int')


    def __mul__(self, obj) -> 'BitBoard':
        if isinstance(obj, int):
            return BitBoard(self.int64 * obj)
        elif isinstance(obj, BitBoard):
            return BitBoard(self.int64 * obj.int64)
        else:
            raise ValueError('Must be an instance of BitBoard class or int')


    def __and__(self, obj) -> 'BitBoard':
        if isinstance(obj, int):
            return BitBoard(self.int64 & obj)
        elif isinstance(obj, BitBoard):
            return BitBoard(self.int64 & obj.int64)
        else:
            raise ValueError('Must be an instance of BitBoard class or int')


    def __or__(self, obj) -> 'BitBoard':
        if isinstance(obj, int):
            return BitBoard(self.int64 | obj)
        elif isinstance(obj, BitBoard):
            return BitBoard(self.int64 | obj.int64)
        else:
            raise ValueError('Must be an instance of BitBoard class or int')


    def __xor__(self, obj) -> 'BitBoard':
        if isinstance(obj, int):
            return BitBoard(self.int64 ^ obj)
        elif isinstance(obj, BitBoard):
            return BitBoard(self.int64 ^ obj.int64)
        else:
            raise ValueError('Must be an instance of BitBoard class or int')


    def __invert__(self) -> 'BitBoard':
        if isinstance(self, BitBoard):
            return BitBoard(~self.int64)
        else:
            raise ValueError('Must be an instance of BitBoard class')


    def __lshift__(self, obj) -> 'BitBoard':
        if isinstance(obj, int):
            return BitBoard(self.int64 << obj)
        elif isinstance(obj, BitBoard):
            return BitBoard(self.int64 << obj.int64)
        else:
            raise ValueError('Must be an instance of BitBoard class or int')


    def __rshift__(self, obj) -> 'BitBoard':
        # perform unsigned right shift
        uint64 = self.int64 + (1 << 64) if self.int64 < 0 else self.int64
        if isinstance(obj, int):
            return BitBoard(uint64 >> obj) if obj != 0 else BitBoard(self.int64)
        elif isinstance(obj, BitBoard):
            return BitBoard(uint64 >> obj.int64) if obj.int64 != 0 else BitBoard(self.int64)
        else:
            raise ValueError('Must be an instance of BitBoard class or int')


class BitBoardMasks():
    # specific bitboard masks
    _file_ab = BitBoard(-4557430888798830400)
    _file_gh = BitBoard(217020518514230019)
    _centre = BitBoard(103481868288)
    _extended_centre = BitBoard(66229406269440)
    _king_side = BitBoard(1085102592571150095)
    _queen_side = BitBoard(-1085102592571150096)
    _king_span_c7 = BitBoard(8093091675687092224) # where c7 king can attack
    _knight_span_c6 = BitBoard(5802888705324613632) # where c6 knight can attack
    _not_allied_pieces = BitBoard() # if in white func: all pieces white can capture (not black king)
    _enemy_pieces = BitBoard() # if in white func: black pieces but no black king
    _empty = BitBoard()
    _occupied = BitBoard()

    # region based bitboard masks
    _rank_masks = [
        BitBoard(-72057594037927936),
        BitBoard(71776119061217280),
        BitBoard(280375465082880),
        BitBoard(1095216660480),
        BitBoard(4278190080),
        BitBoard(16711680),
        BitBoard(65280),
        BitBoard(255),
    ] # from rank 8 to rank 1
    _file_masks = [
        BitBoard(-9187201950435737472),
        BitBoard(4629771061636907072),
        BitBoard(2314885530818453536),
        BitBoard(1157442765409226768),
        BitBoard(578721382704613384),
        BitBoard(289360691352306692),
        BitBoard(144680345676153346),
        BitBoard(72340172838076673),
    ] # from file a to file h
    _diagonal_masks = [
        BitBoard(-9223372036854775808),
        BitBoard(4647714815446351872),
        BitBoard(2323998145211531264),
        BitBoard(1161999622361579520),
        BitBoard(580999813328273408),
        BitBoard(290499906672525312),
        BitBoard(145249953336295424),
        BitBoard(72624976668147840),
        BitBoard(283691315109952),
        BitBoard(1108169199648),
        BitBoard(4328785936),
        BitBoard(16909320),
        BitBoard(66052),
        BitBoard(258),
        BitBoard(1),
    ] # from top left to bottom right
    _anti_diagonal_masks = [
        BitBoard(72057594037927936),
        BitBoard(144396663052566528),
        BitBoard(288794425616760832),
        BitBoard(577588855528488960),
        BitBoard(1155177711073755136),
        BitBoard(2310355422147575808),
        BitBoard(4620710844295151872),
        BitBoard(-9205322385119247871),
        BitBoard(36099303471055874),
        BitBoard(141012904183812),
        BitBoard(550831656968),
        BitBoard(2151686160),
        BitBoard(8405024),
        BitBoard(32832),
        BitBoard(128),
    ] # from top right to bottom left

    # property getters / setters to keep synchronization of the attributes between multiple instances
    # only create setter for mutable data

    @property
    def file_ab(self) -> 'BitBoard':
        return type(self)._file_ab


    @property
    def file_gh(self) -> 'BitBoard':
        return type(self)._file_gh


    @property
    def centre(self) -> 'BitBoard':
        return type(self)._centre


    @property
    def extended_centre(self) -> 'BitBoard':
        return type(self)._extended_centre


    @property
    def king_side(self) -> 'BitBoard':
        return type(self)._king_side


    @property
    def queen_side(self) -> 'BitBoard':
        return type(self)._queen_side


    @property
    def king_span_c7(self) -> 'BitBoard':
        return type(self)._king_span_c7


    @property
    def knight_span_c6(self) -> 'BitBoard':
        return type(self)._knight_span_c6


    @property
    def not_allied_pieces(self) -> 'BitBoard':
        return type(self)._not_allied_pieces


    @not_allied_pieces.setter
    def not_allied_pieces(self, val) -> None:
        if isinstance(val, BitBoard):
            type(self)._not_allied_pieces = val
        else:
            raise ValueError('Set value must be an instance of BitBoard class')


    @property
    def enemy_pieces(self) -> 'BitBoard':
        return type(self)._enemy_pieces


    @enemy_pieces.setter
    def enemy_pieces(self, val) -> None:
        if isinstance(val, BitBoard):
            type(self)._enemy_pieces = val
        else:
            raise ValueError('Set value must be an instance of BitBoard class')


    @property
    def empty(self) -> 'BitBoard':
        return type(self)._empty


    @empty.setter
    def empty(self, val) -> None:
        if isinstance(val, BitBoard):
            type(self)._empty = val
        else:
            raise ValueError('Set value must be an instance of BitBoard class')


    @property
    def occupied(self) -> 'BitBoard':
        return type(self)._occupied


    @occupied.setter
    def occupied(self, val) -> None:
        if isinstance(val, BitBoard):
            type(self)._occupied = val
        else:
            raise ValueError('Set value must be an instance of BitBoard class')


    @property
    def rank_masks(self) -> list['BitBoard']:
        return type(self)._rank_masks


    @property
    def file_masks(self) -> list['BitBoard']:
        return type(self)._file_masks


    @property
    def diagonal_masks(self) -> list['BitBoard']:
        return type(self)._diagonal_masks


    @property
    def anti_diagonal_masks(self) -> list['BitBoard']:
        return type(self)._anti_diagonal_masks
