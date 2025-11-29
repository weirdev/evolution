from dataclasses import dataclass
from .brain import Brain, NeuronType
from .serialization import JsonObject
from .simrand import RANDOM
from .stats import SimStepStats

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

    def step(self, stimulus: dict[int, float], food_quality: float):
        self.brain_state = self.brain.process_n(stimulus, 3)

        output_neuron = self.brain.output_neuron_ids[0]
        consumed_amount = self.brain_state[output_neuron]
        self._body.fullness = consumed_amount

        if food_quality < 0.1 and consumed_amount > 0.2:
            self._body.poisoned = True

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