from .brain import Brain
from .simrand import RANDOM
from .stats import SimStepStats

HUNGER_THRESHOLD = 0.5
FERTILE_THRESHOLD = 0.7
OVEREATEN_THRESHOLD = 1


class Organism:
    def __init__(self, brain: Brain):
        self.brain = brain
        self.brain_state: dict[int, float] = {}
        self._fullness = 1.0

    def step(self, stimulus: dict[int, float]):
        self.brain_state = self.brain.process_n(stimulus, 3)
        self._fullness -= 0.5

    def should_die(self) -> bool:
        if len(self.brain_state) == 0:
            # Baby
            return False

        output_neuron = self.brain.output_neuron_ids[0]
        if self.brain_state[output_neuron] < HUNGER_THRESHOLD:
            if RANDOM.random() < 0.5:
                return True
        if self.brain_state[output_neuron] >= OVEREATEN_THRESHOLD:
            if RANDOM.random() < 0.5:
                return True
        return False

    def should_reproduce(self):
        if len(self.brain_state) == 0:
            # Baby
            return False

        output_neuron = self.brain.output_neuron_ids[0]
        # Can reproduce if eats > FERTILE_THRESHOLD
        if self.brain_state[output_neuron] > FERTILE_THRESHOLD:
            if RANDOM.random() < 0.2:
                return True
        return False

    def get_stats(self, step: int) -> SimStepStats:
        if len(self.brain_state) == 0:
            fit = 1
            fertile = 0
        else:
            output_neuron = self.brain.output_neuron_ids[0]
            fit = 0
            if self.brain_state[output_neuron] >= HUNGER_THRESHOLD:
                fit = 1
            fertile = 0
            if self.brain_state[output_neuron] > FERTILE_THRESHOLD:
                fertile = 1

        return SimStepStats(
            step=step, living_count=1, fit_count=fit, fertile_count=fertile
        )

    def create_baby(self) -> "Organism":
        # Asexual reproduction
        baby_brain = self.brain.deepcopy()
        return Organism(baby_brain)
