from .brain import Brain, NeuronType
from .neuron import Neuron, Edge
from .organism import Organism
from .simrand import RANDOM, SEED
from .stats import SimStepStats, plot_sim_stats


def create_brain() -> Brain:
    brain = Brain()

    add_neurons_to_brain(brain)
    add_edges_to_brain(brain)

    return brain


def add_edges_to_brain(brain: Brain):
    for _ in range(4):
        brain.add_random_edge()


def add_neurons_to_brain(brain: Brain):
    for _ in range(1):
        brain.add_default_neuron(NeuronType.INPUT)

    for _ in range(2):
        brain.add_default_neuron(NeuronType.CONTROL)

    for _ in range(1):
        brain.add_default_neuron(NeuronType.OUTPUT)


def sim():
    print(f"Seed: {SEED}\n")

    organisms = [Organism(create_brain()) for _ in range(100)]

    stats: list[SimStepStats] = []
    for step in range(500):
        food_quality = float(step % 2)
        # Stimulus exactly matches environment
        stimulus = {0: food_quality}

        for organism in organisms:
            organism.step(stimulus, food_quality)

        apply_kills(organisms)
        apply_reproduction(organisms)

        step_stats = SimStepStats.empty(step)
        for o in organisms:
            step_stats += o.get_stats(step)

        stats.append(step_stats)

        print(
            f"After sim step {step_stats.step}, {step_stats.living_count} organisms remaining. {step_stats.fit_count} fit, {step_stats.fertile_count} fertile"
        )

    plot_sim_stats(stats)


def apply_kills(organisms: list[Organism]):
    tokill: set[int] = set()
    for oidx, organism in enumerate(organisms):
        if organism.should_die():
            tokill.add(oidx)

    organisms[:] = [o for i, o in enumerate(organisms) if i not in tokill]


def apply_reproduction(
    organisms: list[Organism],
):
    babies = []
    for oidx, organism in enumerate(organisms):
        if len(babies) + len(organisms) >= 100:
            break
        if organism.should_reproduce():
            # Asexual reproduction
            babies.append(organism.create_baby())

    organisms.extend(babies)


if __name__ == "__main__":
    sim()

"""
Ideas:
1. ~~Eat to live~~
2. ~~Overeating kills~~
3. Bad food signal
4. Perceive presence of food
"""
