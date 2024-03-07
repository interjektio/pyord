import typing

@typing.final
class Rune:
    """Rune
:param num: The rune number"""
    name: str
    num: int

    def __init__(self, /, num: int) -> None:
        """Rune
:param num: The rune number"""

    @staticmethod
    def from_str(s: str) -> Rune:
        """convert the string representation of the rune to a rune
:param s: the string representation of the rune"""

    def __repr__(self, /) -> str:
        """Return repr(self)."""