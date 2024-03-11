from pyord import Runestone, Edict
from bitcointx.core.script import CScript, OP_RETURN, CScriptOp
from bitcointx.core import CTransaction, CTxOut, CTxIn, COutPoint


TX_WITHOUT_RUNESTONE = "0100000001a15d57094aa7a21a28cb20b59aab8fc7d1149a3bdbcddba9c622e4f5f6a99ece010000006c493046022100f93bb0e7d8db7bd46e40132d1f8242026e045f03a0efe71bbb8e3f475e970d790221009337cd7f1f929f00cc6ff01f03729b069a7c21b59b1736ddfee5db5946c5da8c0121033b9b137ee87d5a812d6f506efdd37f0affa7ffc310711c06c7f3e097c9447c52ffffffff0100e1f505000000001976a9140389035a9225b3839e2bbf32d826a1e222031fd888ac00000000"


def test_from_hex_tx_without_runestone():
    r = Runestone.from_hex_tx(TX_WITHOUT_RUNESTONE)
    assert r is None


def test_script_pubkey():
    runestone = Runestone(
        edicts=[
            Edict(
                id=123,
                amount=1000,
                output=0,
            )
        ]
    )
    script_pubkey = runestone.script_pubkey()
    assert isinstance(script_pubkey, bytes)
    script_pubkey = CScript(script_pubkey)
    parts = list(script_pubkey.raw_iter())
    assert len(parts) == 3
    assert parts[0] == (OP_RETURN, None, 0)
    assert parts[1] == (CScriptOp(0x9), b'RUNE_TEST', 1)  # NOTE: RUNE_TEST is expected to change


def test_script_pubkey_round_trip():
    runestone = Runestone(
        edicts=[
            Edict(
                id=123,
                amount=1000,
                output=0,
            )
        ]
    )
    script_pubkey = runestone.script_pubkey()
    tx = CTransaction(
        vin=[
            CTxIn(
                prevout=COutPoint(
                    hash=b'\x00' * 32,
                    n=0
                ),
                scriptSig=CScript()
            )
        ],
        vout=[
            CTxOut(
                nValue=10000,
                scriptPubKey=CScript(script_pubkey),
            )
        ],
    )
    tx_hex = tx.serialize().hex()
    runestone2 = Runestone.from_hex_tx(tx_hex)
    assert runestone == runestone2
