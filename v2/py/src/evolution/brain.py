from typing import Optional
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
        self.labeled_neurons: dict[str, int] = {}

    def add_neuron(
        self, neuron: Neuron, neuron_type: NeuronType, label: Optional[str] = None
    ):
        self._neurons[neuron.id] = neuron
        if neuron_type == NeuronType.INPUT:
            self.input_neuron_ids.append(neuron.id)
        elif neuron_type == NeuronType.CONTROL:
            self.control_neuron_ids.append(neuron.id)
        elif neuron_type == NeuronType.OUTPUT:
            self.output_neuron_ids.append(neuron.id)
        else:
            raise Exception("Unknown NeuronType")

        if label:
            self.labeled_neurons[label] = neuron.id

    def add_edge(self, edge: Edge):
        self._edges.append(edge)

    def process_n(
        self, input_neuron_values: dict[str, float], n: int
    ) -> dict[int, float]:
        neuron_values = {
            self.labeled_neurons[n]: v for n, v in input_neuron_values.items()
        }
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
        new.labeled_neurons = {label: id for label, id in self.labeled_neurons.items()}

        return new

    def add_random_edge(self):
        src = RANDOM.choice(self.input_neuron_ids + self.control_neuron_ids)
        dst = RANDOM.choice(self.control_neuron_ids + self.output_neuron_ids)
        weight = (RANDOM.random() * 4) - 2

        self.add_edge(Edge(src, dst, weight))

    def remove_random_edge(self):
        edge_idx = RANDOM.randrange(len(self._edges))
        self._edges.pop(edge_idx)

    def add_default_neuron(
        self, neuron_type: NeuronType, label: Optional[str] = None
    ) -> int:
        bias = (RANDOM.random() * 2) - 1
        reset_factor = RANDOM.random()
        neuron_id = max(self._neurons, default=-1) + 1
        self.add_neuron(Neuron(neuron_id, bias, reset_factor), neuron_type, label)
        return neuron_id

    def remove_random_neuron(self, neuron_type: NeuronType, autoprune=True):
        if neuron_type == NeuronType.INPUT:
            ids = self.input_neuron_ids
        elif neuron_type == NeuronType.CONTROL:
            ids = self.control_neuron_ids
        elif neuron_type == NeuronType.OUTPUT:
            ids = self.output_neuron_ids
        else:
            raise Exception("Unknown neuron type")
        neuron_idx = RANDOM.randrange(len(ids))
        del self._neurons[ids[neuron_idx]]
        ids.pop(neuron_idx)

        if autoprune:
            self.prune_disconnected_edges()

    def prune_disconnected_edges(self):
        pruned_edges = []
        for edge in self._edges:
            if (edge.source in self._neurons) and (edge.target in self._neurons):
                pruned_edges.append(edge)
        self._edges = pruned_edges

    def to_json(self) -> JsonObject:
        return {
            "neurons": [n.to_json() for n in self._neurons.values()],
            "edges": [e.to_json() for e in self._edges],
            "input_neuron_ids": self.input_neuron_ids,
            "control_neuron_ids": self.control_neuron_ids,
            "output_neuron_ids": self.output_neuron_ids,
            "labeled_neurons": self.labeled_neurons,
        }

    @classmethod
    def from_json(cls, obj: JsonObject) -> "Brain":
        brain = cls()

        neurons = (Neuron.from_json(n) for n in obj["neurons"])
        brain._neurons = {n.id: n for n in neurons}
        brain._edges = [Edge.from_json(e) for e in obj["edges"]]
        brain.input_neuron_ids = obj["input_neuron_ids"]
        brain.control_neuron_ids = obj["control_neuron_ids"]
        brain.output_neuron_ids = obj["output_neuron_ids"]
        brain.labeled_neurons = obj.get("labeled_neurons", {})

        return brain
