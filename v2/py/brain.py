from neuron import Neuron, Edge


class Brain:
    def __init__(self):
        self.neurons: dict[int, Neuron] = {}
        self.edges: list[Edge] = []

    def add_neuron(self, neuron: Neuron):
        self.neurons[neuron.id] = neuron

    def add_edge(self, edge: Edge):
        self.edges.append(edge)

    def process_n(
        self, input_neuron_values: dict[int, float], n: int
    ) -> dict[int, float]:
        neuron_values = input_neuron_values
        for _ in range(n):
            neuron_values = self.step(neuron_values)
        return neuron_values

    def step(self, neuron_values: dict[int, float]) -> dict[int, float]:
        signals: dict[int, list[float]] = {neuron_id: [] for neuron_id in self.neurons}

        for edge in self.edges:
            source_output = neuron_values[edge.source]
            transmitted_signal = edge.transmit(source_output)
            signals[edge.target].append(transmitted_signal)

        neuron_values: dict[int, float] = {
            neuron_id: neuron.activate(signals[neuron_id])
            for neuron_id, neuron in self.neurons.items()
        }

        return neuron_values
