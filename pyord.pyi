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
class Etching:
    divisibility: int
    mint: typing.Optional[Mint]
    rune: typing.Optional[Rune]
    spacers: int
    symbol: typing.Optional[str]

    def __init__(self, /, divisibility: int, spacers: int, mint: typing.Optional[Mint]=None, rune: typing.Optional[Rune]=None, symbol: typing.Optional[str]=None) -> None:...

    def __repr__(self, /) -> str:
        """Return repr(self)."""

@typing.final
class Mint:
    deadline: typing.Optional[int]
    limit: typing.Optional[int]
    term: typing.Optional[int]

    def __init__(self, /, deadline: typing.Optional[int]=None, limit: typing.Optional[int]=None, term: typing.Optional[int]=None) -> None:...

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
class RuneId:
    """RuneId
:param height: Etching block height
:param index: Etching transaction index"""
    height: int
    index: int
    num: int
    'number suitable for use as Edict id'

    def __init__(self, /, height: int, index: int) -> None:
        """RuneId
:param height: Etching block height
:param index: Etching transaction index"""

    @staticmethod
    def from_num(num: int) -> RuneId:
        """Parse the RuneId from a number usable as Edict id"""

    def __eq__(self, value: typing.Any, /) -> bool:
        """Return self==value."""

    def __ge__(self, value: typing.Any, /) -> bool:
        """Return self>=value."""

    def __gt__(self, value: typing.Any, /) -> bool:
        """Return self>value."""

    def __le__(self, value: typing.Any, /) -> bool:
        """Return self<=value."""

    def __lt__(self, value: typing.Any, /) -> bool:
        """Return self<value."""

    def __ne__(self, value: typing.Any, /) -> bool:
        """Return self!=value."""

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

    def __init__(self, /, burn: bool=False, edicts: typing.Iterable[Edict]=(), claim: typing.Optional[int]=None, default_output: typing.Optional[int]=None, etching: typing.Optional[Etching]=None) -> None:
        """Runestone"""

    @staticmethod
    def from_hex_tx(hex_tx: str) -> typing.Optional[Runestone]:
        """Return a Runestone from a Bitcoin transaction, or None if the transaction contains no
Runestone"""

    def script_pubkey(self, /) -> bytes:
        """get the scriptPubKey of the Runestone"""

    def __eq__(self, value: typing.Any, /) -> bool:
        """Return self==value."""

    def __ge__(self, value: typing.Any, /) -> bool:
        """Return self>=value."""

    def __gt__(self, value: typing.Any, /) -> bool:
        """Return self>value."""

    def __le__(self, value: typing.Any, /) -> bool:
        """Return self<=value."""

    def __lt__(self, value: typing.Any, /) -> bool:
        """Return self<value."""

    def __ne__(self, value: typing.Any, /) -> bool:
        """Return self!=value."""

    def __repr__(self, /) -> str:
        """Return repr(self)."""