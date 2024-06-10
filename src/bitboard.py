"""
Define what a bitboard is and functions to operate on them.
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
                ValueError('Binary string must contain 64 bits')
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
            raise ValueError('Must be an instance of BitBoard class')


    def __lt__(self, obj) -> bool:
        if isinstance(obj, int):
            return self.int64 < obj
        elif isinstance(obj, BitBoard):
            return self.int64 < obj.int64
        else:
            raise ValueError('Must be an instance of BitBoard class')


    def __gt__(self, obj) -> bool:
        if isinstance(obj, int):
            return self.int64 > obj
        elif isinstance(obj, BitBoard):
            return self.int64 > obj.int64
        else:
            raise ValueError('Must be an instance of BitBoard class')


    def __le__(self, obj) -> bool:
        if isinstance(obj, int):
            return self.int64 <= obj
        elif isinstance(obj, BitBoard):
            return self.int64 <= obj.int64
        else:
            raise ValueError('Must be an instance of BitBoard class')


    def __ge__(self, obj) -> bool:
        if isinstance(obj, int):
            return self.int64 >= obj
        elif isinstance(obj, BitBoard):
            return self.int64 >= obj.int64
        else:
            raise ValueError('Must be an instance of BitBoard class')


    def __add__(self, obj) -> 'BitBoard':
        if isinstance(obj, int):
            return BitBoard(self.int64 + obj)
        elif isinstance(obj, BitBoard):
            return BitBoard(self.int64 + obj.int64)
        else:
            raise ValueError('Must be an instance of BitBoard class')


    def __sub__(self, obj) -> 'BitBoard':
        if isinstance(obj, int):
            return BitBoard(self.int64 - obj)
        elif isinstance(obj, BitBoard):
            return BitBoard(self.int64 - obj.int64)
        else:
            raise ValueError('Must be an instance of BitBoard class')


    def __mul__(self, obj) -> 'BitBoard':
        if isinstance(obj, int):
            return BitBoard(self.int64 * obj)
        elif isinstance(obj, BitBoard):
            return BitBoard(self.int64 * obj.int64)
        else:
            raise ValueError('Must be an instance of BitBoard class')


    def __and__(self, obj) -> 'BitBoard':
        if isinstance(obj, int):
            return BitBoard(self.int64 & obj)
        elif isinstance(obj, BitBoard):
            return BitBoard(self.int64 & obj.int64)
        else:
            raise ValueError('Must be an instance of BitBoard class')


    def __or__(self, obj) -> 'BitBoard':
        if isinstance(obj, int):
            return BitBoard(self.int64 | obj)
        elif isinstance(obj, BitBoard):
            return BitBoard(self.int64 | obj.int64)
        else:
            raise ValueError('Must be an instance of BitBoard class')


    def __xor__(self, obj) -> 'BitBoard':
        if isinstance(obj, int):
            return BitBoard(self.int64 ^ obj)
        elif isinstance(obj, BitBoard):
            return BitBoard(self.int64 ^ obj.int64)
        else:
            raise ValueError('Must be an instance of BitBoard class')


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
            raise ValueError('Must be an instance of BitBoard class')


    def __rshift__(self, obj) -> 'BitBoard':
        # perform unsigned right shift
        uint64 = self.int64 + (1 << 64) if self.int64 < 0 else self.int64
        if isinstance(obj, int):
            return BitBoard(uint64 >> obj) if obj != 0 else BitBoard(self.int64)
        elif isinstance(obj, BitBoard):
            return BitBoard(uint64 >> obj.int64) if obj.int64 != 0 else BitBoard(self.int64)
        else:
            raise ValueError('Must be an instance of BitBoard class')
