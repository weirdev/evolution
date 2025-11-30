import typing
from typing import Callable
from enum import Enum

# 6-binary input 3-binary output
# 3-bits -> 3-bits -> 3-bits
# [0-8) -> [0-8) -> [0-8)
# [0-8) -> [Op0, Op1, ..., Op7] -> [0-8)


def validate_0_7_int(inp: int):
    assert inp >= 0
    assert inp < 8


class I6bOb3Op(Enum):
    Add2Mod8 = (0,)
    Add3Mod8 = (1,)
    Mul2Mod8 = (2,)
    Mul3Mod8 = (3,)
    Pow2Mod8 = (4,)
    Pow3Mod8 = (5,)
    NegMod8 = (6,)
    Mul5Mod8 = (7,)

    @classmethod
    def from_inp(cls, opinp: int) -> "I6bOb3Op":
        validate_0_7_int(opinp)
        return cls(opinp)

    def get_op_fn(self) -> Callable[[int], int]:
        match self:
            case I6bOb3Op.Add2Mod8:
                return add2mod8
            case I6bOb3Op.Add3Mod8:
                return add3mod8
            case I6bOb3Op.Mul2Mod8:
                return mul2mod8
            case I6bOb3Op.Mul3Mod8:
                return mul3mod8
            case I6bOb3Op.Pow2Mod8:
                return pow2mod8
            case I6bOb3Op.Pow3Mod8:
                return pow3mod8
            case I6bOb3Op.NegMod8:
                return neg_mod8
            case I6bOb3Op.Mul5Mod8:
                return mul5mod8
            case _:
                raise Exception("Unknown Op")


def add2mod8(inp: int) -> int:
    validate_0_7_int(inp)
    return (inp + 2) % 8


def add3mod8(inp: int) -> int:
    validate_0_7_int(inp)
    return (inp + 2) % 8


def mul2mod8(inp: int) -> int:
    validate_0_7_int(inp)
    return (inp * 2) % 8


def mul3mod8(inp: int) -> int:
    validate_0_7_int(inp)
    return (inp * 3) % 8


def pow2mod8(inp: int) -> int:
    validate_0_7_int(inp)
    return (inp**2) % 8


def pow3mod8(inp: int) -> int:
    validate_0_7_int(inp)
    return (inp**3) % 8


def neg_mod8(inp: int) -> int:
    validate_0_7_int(inp)
    return (-inp) % 8


def mul5mod8(inp: int) -> int:
    validate_0_7_int(inp)
    return (inp * 5) % 8


def get_correct_output(inp: int, opinp: int) -> int:
    op = I6bOb3Op.from_inp(opinp)
    return op.get_op_fn()(inp)


def int_to_neuron_pattern(inp: int) -> tuple[float, float, float]:
    validate_0_7_int(inp)
    pattern = [0.0, 0.0, 0.0]
    for i in range(3):
        # Little endian
        pattern[i] = float(inp % 2)
        inp >>= 1
    return typing.cast(tuple[float, float, float], tuple(pattern))


def pattern_to_int(output: tuple[float, float, float]) -> int:
    val = 0
    for i in range(3):
        val <<= 1
        # Little endian
        val += round(output[-i - 1])

    validate_0_7_int(val)
    return val