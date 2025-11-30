from dataclasses import dataclass
from .brain import Brain, NeuronType
from .serialization import JsonObject
from .simrand import RANDOM
from .stats import SimStepStats
from .training_functions import get_correct_output, pattern_to_int

HUNGER_THRESHOLD = 0.5
FERTILE_THRESHOLD = 0.7
OVEREATEN_THRESHOLD = 1


@dataclass
class Body:
    fullness: float = 1.0
    poisoned: bool = False


class Organism:
    def __init__(self, brain: Brain):
        self.brain = brain
        self.brain_state: dict[int, float] = {}
        self._body = Body()

    def step(self, stimulus: dict[str, float], food_quality: float):
        self.brain_state = self.brain.process_n(stimulus, 3)

        # Eating training
        output_eat = self.brain.labeled_neurons["output_eat"]
        consumed_amount = self.brain_state[output_eat]
        self._body.fullness = consumed_amount

        if food_quality < 0.1 and consumed_amount > 0.2:
            self._body.poisoned = True

        # Int relation training
        input_int_pattern = (
            stimulus["input_int_arg_b0"],
            stimulus["input_int_arg_b1"],
            stimulus["input_int_arg_b2"],
        )
        input_int = pattern_to_int(input_int_pattern)

        input_op_pattern = (
            stimulus["input_int_op_b0"],
            stimulus["input_int_op_b1"],
            stimulus["input_int_op_b2"],
        )
        op_int = pattern_to_int(input_op_pattern)
        expected_int_output = get_correct_output(inp=input_int, opinp=op_int)

        output_int_pattern = (
            self.brain_state[self.brain.labeled_neurons["output_int_result_b0"]],
            self.brain_state[self.brain.labeled_neurons["output_int_result_b1"]],
            self.brain_state[self.brain.labeled_neurons["output_int_result_b2"]],
        )
        actual_int_output = pattern_to_int(output_int_pattern)
        # TODO: Affect fitness based on expected vs actual int output

    def should_die(self) -> bool:
        if len(self.brain_state) == 0:
            # Baby
            return False

        if self._body.poisoned:
            if RANDOM.random() < 0.14:
                return True

        if self._body.fullness < HUNGER_THRESHOLD:
            if RANDOM.random() < 0.05:
                return True
        if self._body.fullness >= OVEREATEN_THRESHOLD:
            if RANDOM.random() < 0.05:
                return True
        return False

    def should_reproduce(self):
        if len(self.brain_state) == 0:
            # Baby
            return False

        if self._body.fullness > FERTILE_THRESHOLD:
            if RANDOM.random() < 0.17:
                return True
        return False

    def get_stats(self, step: int) -> SimStepStats:
        if len(self.brain_state) == 0:
            fit = 1
            fertile = 0
        else:
            fit = 0
            if self._body.fullness >= HUNGER_THRESHOLD:
                fit = 1
            fertile = 0
            if self._body.fullness > FERTILE_THRESHOLD:
                fertile = 1

        poisoned = 1 if self._body.poisoned else 0

        return SimStepStats(
            step=step,
            living_count=1,
            fit_count=fit,
            fertile_count=fertile,
            poisoned_count=poisoned,
        )

    def create_baby(self) -> "Organism":
        # Asexual reproduction
        baby_brain = self.brain.deepcopy()

        # Evolution
        if len(baby_brain._neurons) < 10:
            for _ in range(4):
                baby_brain.add_default_neuron(NeuronType.CONTROL)
            for _ in range(12):
                baby_brain.add_random_edge()
        # TODO: Add and remove neurons / connections during evolution

        return Organism(baby_brain)

    def to_json(self) -> JsonObject:
        return {
            "brain": self.brain.to_json(),
        }

    @classmethod
    def from_json(cls, obj: JsonObject) -> "Organism":
        return cls(Brain.from_json(obj["brain"]))
