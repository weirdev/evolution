from dataclasses import dataclass
from .brain import Brain
from .simrand import RANDOM
from .stats import SimStepStats

HUNGER_THRESHOLD = 0.5
FERTILE_THRESHOLD = 0.7
OVEREATEN_THRESHOLD = 1


@dataclass
class Body:
    fullness: float = 1.0
    poisioned: bool = False


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
            self._body.poisioned = True

    def should_die(self) -> bool:
        if len(self.brain_state) == 0:
            # Baby
            return False

        if self._body.poisioned:
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

        poisioned = 1 if self._body.poisioned else 0

        return SimStepStats(
            step=step,
            living_count=1,
            fit_count=fit,
            fertile_count=fertile,
            poisioned_count=poisioned,
        )

    def create_baby(self) -> "Organism":
        # Asexual reproduction
        baby_brain = self.brain.deepcopy()

        # Evolution
        # baby_brain.add_neuron()

        return Organism(baby_brain)
