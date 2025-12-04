from typing import Optional

from .brain import Brain, NeuronType
from .environment import Environment
from .serialization import JsonObject
from .simrand import RANDOM
from .stats import SimStepStats
from .training_functions import get_correct_output, pattern_to_int

HUNGER_THRESHOLD = 0.5
FERTILE_THRESHOLD = 0.7
OVEREATEN_THRESHOLD = 1
MAX_NEURONS = 45  # 10 works for 1000 iterations
BRAIN_PROCESSING_STEPS = 4
MASTER_KILL_FACTOR = 0.45
MASTER_EVOLUTION_RATE = 0.4  # 0.25
REPRODUCTION_FACTOR = 0.09  # 0.17


class Body:
    def __init__(
        self,
        fullness: float = 1.0,
        poisoned: bool = False,
        int_calc_results: Optional[list[bool]] = None,
    ):
        self.fullness = fullness
        self.poisoned = poisoned
        if int_calc_results is None:
            self.int_calc_results: list[bool] = []
        else:
            self.int_calc_results = int_calc_results


class Organism:
    def __init__(self, brain: Brain):
        self.brain = brain
        self.brain_state: dict[int, float] = {}
        self._body = Body()

    def step(self, stimulus: dict[str, float], env: Environment):
        self.brain_state = self.brain.process_n(stimulus, BRAIN_PROCESSING_STEPS)

        # Eating training
        output_eat = self.brain.labeled_neurons["output_eat"]
        consumed_amount = self.brain_state[output_eat]
        self._body.fullness = consumed_amount

        if env.food_quality < 0.1 and consumed_amount > 0.2:
            self._body.poisoned = True

        # Int relation training
        expected_int_output = get_correct_output(
            inp=env.input_int_arg, opinp=env.input_int_op
        )
        # Debug
        assert env.input_int_op == 0

        output_int_pattern = (
            self.brain_state[self.brain.labeled_neurons["output_int_result_b0"]],
            self.brain_state[self.brain.labeled_neurons["output_int_result_b1"]],
            self.brain_state[self.brain.labeled_neurons["output_int_result_b2"]],
        )
        actual_int_output = pattern_to_int(output_int_pattern)
        if env.display_kill_debug:
            print(
                f"Expected int output: {expected_int_output}. Actual int output: {actual_int_output}"
            )
        self._body.int_calc_results.append(expected_int_output == actual_int_output)

    def should_die(self) -> bool:
        if len(self.brain_state) == 0:
            # Baby
            return False

        if self._body.poisoned:
            if RANDOM.random() < 0.14 * MASTER_KILL_FACTOR:
                return True

        if self._body.fullness < HUNGER_THRESHOLD:
            if RANDOM.random() < 0.05 * MASTER_KILL_FACTOR:
                return True
        if self._body.fullness >= OVEREATEN_THRESHOLD:
            if RANDOM.random() < 0.05 * MASTER_KILL_FACTOR:
                return True
        if self._body.int_calc_results and not self._body.int_calc_results[-1]:
            if RANDOM.random() < 0.05 * MASTER_KILL_FACTOR: # 0.129
                return True
        # Big brain penalty (max risk = 1%)
        # if (
        #     RANDOM.random()
        #     < ((len(self.brain._neurons) / MAX_NEURONS) / 100) * MASTER_KILL_FACTOR
        # ):
        #     return True

        return False

    def should_reproduce(self):
        if len(self.brain_state) == 0:
            # Baby
            return False

        if self._body.fullness > FERTILE_THRESHOLD:
            if RANDOM.random() < REPRODUCTION_FACTOR:
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

        if RANDOM.random() < 0.4 and len(baby_brain._neurons) < MAX_NEURONS:
            if RANDOM.random() < 0.3 * MASTER_EVOLUTION_RATE:
                for _ in range(1):
                    baby_brain.add_default_neuron(NeuronType.CONTROL)
        else:
            if RANDOM.random() < 0.3 * MASTER_EVOLUTION_RATE:
                for _ in range(1):
                    baby_brain.remove_random_neuron(NeuronType.CONTROL, autoprune=False)
                baby_brain.prune_disconnected_edges()

            # TODO: We will want our brain less connected than this eventually
            if (
                RANDOM.random() < 0.4
                and len(baby_brain._edges) < (len(baby_brain._neurons) ** 2) // 2
            ):
                if RANDOM.random() < 0.6 * MASTER_EVOLUTION_RATE:
                    for _ in range(max(len(baby_brain._edges) // 20, 1)):
                        baby_brain.add_random_edge()
            else:
                if RANDOM.random() < 0.6 * MASTER_EVOLUTION_RATE:
                    for _ in range(max(len(baby_brain._edges) // 20, 1)):
                        baby_brain.remove_random_edge()
        # TODO: Add and remove neurons / connections during evolution

        return Organism(baby_brain)

    def to_json(self) -> JsonObject:
        return {
            "brain": self.brain.to_json(),
        }

    @classmethod
    def from_json(cls, obj: JsonObject) -> "Organism":
        return cls(Brain.from_json(obj["brain"]))
