from dataclasses import dataclass


@dataclass
class Environment:
    food_quality: float
    input_int_arg: int
    input_int_op: int
    display_kill_debug: bool
