import random

from .neuron import Neuron, Edge
from .brain import Brain, NeuronType


RANDOM_UNSEEDED = random.Random()
SEED = RANDOM_UNSEEDED.randrange(1000)
RANDOM = random.Random(SEED)


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

    brain_states = []
    for step in range(500):
        stimulus = {0: 1.0}

        brain_states = []
        for brain in brains:
            brain_states.append(brain.process_n(stimulus, 3))

        tokill = set()
        for bidx, (brain, bstate) in enumerate(zip(brains, brain_states, strict=True)):
            output_neuron = brain.output_neuron_ids[0]
            if bstate[output_neuron] < 0.8:
                if RANDOM.random() < 0.3:
                    tokill.add(bidx)

        brains = [b for i, b in enumerate(brains) if i not in tokill]
        brain_states = [s for i, s in enumerate(brain_states) if i not in tokill]

        living_count = len(brains)

        print(f"After sim step {step}, {living_count} organisms remaining")


if __name__ == "__main__":
    sim()

"""
Ideas:
1. Eat to live
2. Overeating kills
3. Perceive presence of food
"""
