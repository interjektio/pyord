import typing

@typing.final
class Edict:
    amount: int
    id: int
    output: int

    def __init__(self, /, id: int, amount: int, output: int) -> None:...

    def __repr__(self, /) -> str:
        """Return repr(self)."""

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

@typing.final
class Runestone:
    """Runestone"""
    burn: bool
    claim: typing.Optional[int]
    default_output: typing.Optional[int]
    edicts: typing.List[Edict]
    etching: typing.Optional[Etching]

    def __init__(self, /, burn: bool=False, edicts: typing.Iterable[Edict]=(), claim: typing.Optional[int]=None, default_output: typing.Optional[int]=None) -> None:
        """Runestone"""

    def __repr__(self, /) -> str:
        """Return repr(self)."""