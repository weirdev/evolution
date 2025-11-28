class Neuron:
    def __init__(self, id: int, bias: float, reset_factor: float):
        self.id = id
        self.bias = bias
        self.reset_factor = reset_factor
        self.activation: float = 0.0

    def activate(self, input_signals: list[float]) -> float:
        total_input = sum(input_signals) + self.bias + self.activation
        output = self._relu(total_input)
        self.activation = output * self.reset_factor
        return output

    def _relu(self, x: float) -> float:
        return min(max(0.0, x), 1.0)

    def deepcopy(self) -> "Neuron":
        return Neuron(self.id, self.bias, self.reset_factor)


class Edge:
    def __init__(self, source: int, target: int, weight: float):
        self.source = source
        self.target = target
        self.weight = weight

    def transmit(self, signal: float) -> float:
        return signal * self.weight

    def deepcopy(self) -> "Edge":
        return Edge(self.source, self.target, self.weight)
