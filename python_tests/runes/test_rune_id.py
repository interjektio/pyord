import pytest
from pyord import RuneId


def test_rune_id_to_128():
    rune_id = RuneId(
        height=3,
        index=1,
    )
    assert rune_id.num == 0b11_0000_0000_0000_0001


def test_repr():
    rune_id = RuneId(
        height=1,
        index=2,
    )
    assert repr(rune_id) == "RuneId(height=1, index=2)"


def test_from_num():
    assert RuneId.from_num(0x060504030201) == RuneId(
        height=0x06050403,
        index=0x0201,
    )
    with pytest.raises(ValueError):
        RuneId.from_num(0x07060504030201)