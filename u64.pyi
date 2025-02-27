from typing import Optional

class u64:
    def __new__(cls, value: int) -> 'u64': ...
    def __abs__(self) -> 'u64': ...
    def __add__(self, other: 'u64') -> 'u64': ...
    def __and__(self, other: 'u64') -> 'u64': ...
    def __bool__(self) -> bool: ...
    def __ceil__(self) -> 'u64': ...
    def __divmod__(self, other: 'u64') -> 'tuple[u64, u64]': ...
    def __float__(self) -> float: ...
    def __floor__(self) -> 'u64': ...
    def __floordiv__(self, other: 'u64') -> 'u64': ...
    def __index__(self) -> int: ...
    def __int__(self) -> int: ...
    def __invert__(self) -> 'u64': ...
    def __lshift__(self, other: 'u64') -> 'u64': ...
    def __mod__(self, other: 'u64') -> 'u64': ...
    def __mul__(self, other: 'u64') -> 'u64': ...
    def __or__(self, other: 'u64') -> 'u64': ...
    def __pos__(self) -> 'u64': ...
    def __pow__(self, other: 'u64', modulo: 'Optional[u64]' = None) -> 'u64': ...
    def __repr__(self) -> str: ...
    def __round__(self) -> 'u64': ...
    def __rshift__(self, other: 'u64') -> 'u64': ...
    def __sub__(self, other: 'u64') -> 'u64': ...
    def __truediv__(self, other: 'u64') -> 'u64': ...
    def __trunc__(self) -> 'u64': ...
    def __xor__(self, other: 'u64') -> 'u64': ...
