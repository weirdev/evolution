from enum import Enum

from .neuron import Neuron, Edge
from .serialization import JsonObject
from .simrand import RANDOM


class NeuronType(Enum):
    INPUT = (0,)
    CONTROL = (1,)
    OUTPUT = 2


class Brain:
    def __init__(self):
        self._neurons: dict[int, Neuron] = {}
        self._edges: list[Edge] = []
        self.input_neuron_ids: list[int] = []
        self.control_neuron_ids: list[int] = []
        self.output_neuron_ids: list[int] = []

    def add_neuron(self, neuron: Neuron, neuron_type: NeuronType):
        self._neurons[neuron.id] = neuron
        if neuron_type == NeuronType.INPUT:
            self.input_neuron_ids.append(neuron.id)
        elif neuron_type == NeuronType.CONTROL:
            self.control_neuron_ids.append(neuron.id)
        elif neuron_type == NeuronType.OUTPUT:
            self.output_neuron_ids.append(neuron.id)
        else:
            raise Exception("Unknown NeuronType")

    def add_edge(self, edge: Edge):
        self._edges.append(edge)

    def process_n(
        self, input_neuron_values: dict[int, float], n: int
    ) -> dict[int, float]:
        neuron_values = input_neuron_values
        for _ in range(n):
            neuron_values = self._step(neuron_values)
        return neuron_values

    def _step(self, neuron_values: dict[int, float]) -> dict[int, float]:
        signals: dict[int, list[float]] = {neuron_id: [] for neuron_id in self._neurons}

        for edge in self._edges:
            source_output = neuron_values.get(edge.source, 0.0)
            transmitted_signal = edge.transmit(source_output)
            signals[edge.target].append(transmitted_signal)

        neuron_values: dict[int, float] = {
            neuron_id: neuron.activate(signals[neuron_id])
            for neuron_id, neuron in self._neurons.items()
        }

        return neuron_values

    def deepcopy(self) -> "Brain":
        new = Brain()
        new._neurons = {id: n.deepcopy() for id, n in self._neurons.items()}
        new._edges = [e.deepcopy() for e in self._edges]
        new.input_neuron_ids = [id for id in self.input_neuron_ids]
        new.control_neuron_ids = [id for id in self.control_neuron_ids]
        new.output_neuron_ids = [id for id in self.output_neuron_ids]

        return new

    def add_random_edge(self):
        src = RANDOM.choice(self.input_neuron_ids + self.control_neuron_ids)
        dst = RANDOM.choice(self.control_neuron_ids + self.output_neuron_ids)
        weight = (RANDOM.random() * 4) - 2

        self.add_edge(Edge(src, dst, weight))

    def add_default_neuron(self, neuron_type: NeuronType):
        bias = (RANDOM.random() * 2) - 1
        reset_factor = RANDOM.random()
        self.add_neuron(
            Neuron(max(self._neurons, default=-1) + 1, bias, reset_factor), neuron_type
        )

    def to_json(self) -> JsonObject:
        return {
            "neurons": [n.to_json() for n in self._neurons.values()],
            "edges": [e.to_json() for e in self._edges],
            "input_neuron_ids": self.input_neuron_ids,
            "control_neuron_ids": self.control_neuron_ids,
            "output_neuron_ids": self.output_neuron_ids,
        }
