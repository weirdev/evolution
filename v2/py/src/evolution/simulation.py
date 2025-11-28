from dataclasses import dataclass
import random

from .neuron import Neuron, Edge
from .brain import Brain, NeuronType


RANDOM_UNSEEDED = random.Random()
SEED = RANDOM_UNSEEDED.randrange(1000)
RANDOM = random.Random(SEED)

HUNGER_THRESHOLD = 0.5
FERTILE_THRESHOLD = 0.7
OVEREATEN_THRESHOLD = 1


@dataclass
class SimStepStats:
    step: int
    living_count: int
    fit_count: int
    fertile_count: int


def create_brain() -> Brain:
    brain = Brain()

    add_neurons_to_brain(brain)
    add_edges_to_brain(brain)

    return brain


def add_edges_to_brain(brain):
    for _ in range(4):
        src = RANDOM.choice(brain.input_neuron_ids + brain.control_neuron_ids)
        dst = RANDOM.choice(brain.control_neuron_ids + brain.output_neuron_ids)
        weight = (RANDOM.random() * 4) - 2

        brain.add_edge(Edge(src, dst, weight))


def add_neurons_to_brain(brain: Brain):
    neuron_id = 0
    for _ in range(1):
        brain.add_neuron(create_neuron(neuron_id), NeuronType.INPUT)
        neuron_id += 1

    for _ in range(2):
        brain.add_neuron(create_neuron(neuron_id), NeuronType.CONTROL)
        neuron_id += 1

    for _ in range(1):
        brain.add_neuron(create_neuron(neuron_id), NeuronType.OUTPUT)
        neuron_id += 1


def create_neuron(neuron_id) -> Neuron:
    bias = (RANDOM.random() * 2) - 1
    reset_factor = RANDOM.random()
    return Neuron(neuron_id, bias, reset_factor)


def sim():
    print(f"Seed: {SEED}\n")

    brains = [create_brain() for _ in range(100)]

    stats: list[SimStepStats] = []
    brain_states: list[dict[int, float]] = []
    for step in range(500):
        stimulus = {0: 1.0}

        brain_states = []
        for brain in brains:
            brain_states.append(brain.process_n(stimulus, 3))

        apply_kills(brains, brain_states)
        apply_reproduction(brains, brain_states, stimulus)

        step_stats = SimStepStats(
            step=step, living_count=len(brains), fit_count=0, fertile_count=0
        )
        for bidx, (brain, bstate) in enumerate(zip(brains, brain_states, strict=True)):
            output_neuron = brain.output_neuron_ids[0]
            # May die if eats < HUNGER_THRESHOLD
            if bstate[output_neuron] >= HUNGER_THRESHOLD:
                step_stats.fit_count += 1
            # Can reproduce if eats > FERTILE_THRESHOLD
            if bstate[output_neuron] > FERTILE_THRESHOLD:
                step_stats.fertile_count += 1
        stats.append(step_stats)

        print(
            f"After sim step {step_stats.step}, {step_stats.living_count} organisms remaining. {step_stats.fit_count} fit, {step_stats.fertile_count} fertile"
        )

    plot_sim_stats(stats)


def apply_kills(brains: list[Brain], brain_states: list[dict[int, float]]):
    tokill = set()
    for bidx, (brain, bstate) in enumerate(zip(brains, brain_states, strict=True)):
        output_neuron = brain.output_neuron_ids[0]
        # May die if eats < HUNGER_THRESHOLD
        if bstate[output_neuron] < HUNGER_THRESHOLD:
            if RANDOM.random() < 0.5:
                tokill.add(bidx)
        if bstate[output_neuron] >= OVEREATEN_THRESHOLD:
            if RANDOM.random() < 0.5:
                tokill.add(bidx)

    brains[:] = [b for i, b in enumerate(brains) if i not in tokill]
    brain_states[:] = [s for i, s in enumerate(brain_states) if i not in tokill]


def apply_reproduction(
    brains: list[Brain],
    brain_states: list[dict[int, float]],
    stimulus: dict[int, float],
):
    babies = []
    for bidx, (brain, bstate) in enumerate(zip(brains, brain_states, strict=True)):
        if len(babies) + len(brains) >= 100:
            break
        output_neuron = brain.output_neuron_ids[0]
        # Can reproduce if eats > FERTILE_THRESHOLD
        if bstate[output_neuron] > FERTILE_THRESHOLD:
            if RANDOM.random() < 0.2:
                # Asexual reproduction
                baby = brain.deepcopy()
                babies.append((baby, baby.process_n(stimulus, 3)))

    for baby, bstate in babies:
        brains.append(baby)
        brain_states.append(bstate)


def plot_sim_stats(stats: list[SimStepStats], save_path: str | None = None) -> None:
    import matplotlib.pyplot as plt

    if not stats:
        return

    steps = [s.step for s in stats]
    living = [s.living_count for s in stats]
    fit = [s.fit_count for s in stats]
    fertile = [s.fertile_count for s in stats]

    plt.figure(figsize=(10, 5))
    plt.plot(steps, living, label="living_count", linewidth=2)
    plt.plot(steps, fit, label="fit_count", linewidth=2)
    plt.plot(steps, fertile, label="fertile_count", linewidth=2)
    plt.xlabel("Simulation step")
    plt.ylabel("Count")
    plt.title("Simulation counts over time")
    plt.legend(loc="upper right")
    plt.grid(alpha=0.3)
    plt.tight_layout()

    if save_path:
        plt.savefig(save_path)
    else:
        plt.show()

    plt.close()


if __name__ == "__main__":
    sim()

"""
Ideas:
1. ~~Eat to live~~
2. ~~Overeating kills~~
3. Perceive presence of food
"""
