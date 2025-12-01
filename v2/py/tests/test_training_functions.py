import sys
import pathlib

# Add src to sys.path so `evolution` package is importable
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1] / "src"))

from evolution.training_functions import (
    I6bOb3Op,
    add2mod8,
    add3mod8,
    mul2mod8,
    mul3mod8,
    pow2mod8,
    pow3mod8,
    neg_mod8,
    mul5mod8,
    get_correct_output,
    int_to_neuron_pattern,
    pattern_to_int,
    validate_0_7_int,
)


def test_op_functions_against_expected_results():
    # expected mapping for operations (0..7)
    expected_ops = {
        0: lambda x: (x + 2) % 8,  # Add2Mod8
        1: lambda x: (x + 3) % 8,  # Add3Mod8
        2: lambda x: (x * 2) % 8,  # Mul2Mod8
        3: lambda x: (x * 3) % 8,  # Mul3Mod8
        4: lambda x: (x**2) % 8,   # Pow2Mod8
        5: lambda x: (x**3) % 8,   # Pow3Mod8
        6: lambda x: (-x) % 8,     # NegMod8
        7: lambda x: (x * 5) % 8,  # Mul5Mod8
    }

    # For each op id test all inputs 0..7
    for op_id, expected_fn in expected_ops.items():
        for inp in range(8):
            # Using get_correct_output which should apply the operation
            got = get_correct_output(inp, op_id)
            assert got == expected_fn(inp), (
                f"Operation {op_id} failed for input {inp}: got {got}, expected {expected_fn(inp)}"
            )


def test_individual_functions_match_expected():
    # Cross-check individual functions as a sanity check
    for i in range(8):
        assert add2mod8(i) == (i + 2) % 8
        assert add3mod8(i) == (i + 3) % 8
        assert mul2mod8(i) == (i * 2) % 8
        assert mul3mod8(i) == (i * 3) % 8
        assert pow2mod8(i) == (i**2) % 8
        assert pow3mod8(i) == (i**3) % 8
        assert neg_mod8(i) == (-i) % 8
        assert mul5mod8(i) == (i * 5) % 8


def test_int_pattern_roundtrip():
    # int -> pattern -> int should be identity for all valid inputs
    for i in range(8):
        pattern = int_to_neuron_pattern(i)
        # ensure pattern is 3-length tuple of floats
        assert isinstance(pattern, tuple)
        assert len(pattern) == 3
        assert all(isinstance(p, float) for p in pattern)

        got = pattern_to_int(pattern)
        assert got == i


def test_validate_0_7_int_raises_on_invalid_inputs():
    import pytest

    with pytest.raises(AssertionError):
        validate_0_7_int(-1)

    with pytest.raises(AssertionError):
        validate_0_7_int(8)


def test_enum_from_inp_invalid_raises():
    import pytest

    with pytest.raises(AssertionError):
        # from_inp uses validate_0_7_int
        I6bOb3Op.from_inp(-1)

    with pytest.raises(AssertionError):
        I6bOb3Op.from_inp(8)
