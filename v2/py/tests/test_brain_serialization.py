import sys
import pathlib

# Add src to sys.path so `evolution` package is importable
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1] / "src"))

from evolution.brain import Brain, NeuronType
from evolution.neuron import Neuron, Edge


def test_brain_to_json():
    brain = Brain()
    # manually create neurons so biases and reset values are stable
    n0 = Neuron(id=0, bias=0.1, reset_factor=0.2)
    n1 = Neuron(id=1, bias=-0.1, reset_factor=0.3)

    brain.add_neuron(n0, NeuronType.INPUT)
    brain.add_neuron(n1, NeuronType.OUTPUT)

    e = Edge(source=0, target=1, weight=0.5)
    brain.add_edge(e)

    json_obj = brain.to_json()

    assert isinstance(json_obj, dict)
    assert set(json_obj.keys()) == {
        "neurons",
        "edges",
        "input_neuron_ids",
        "control_neuron_ids",
        "output_neuron_ids",
        "labeled_neurons",
    }

    # neurons order is not guaranteed; verify contents instead
    neurons_json = list(json_obj["neurons"])
    assert {tuple(sorted(n.items())) for n in neurons_json} == {
        tuple(sorted(n0.to_json().items())),
        tuple(sorted(n1.to_json().items())),
    }

    assert json_obj["edges"] == [e.to_json()]
    assert json_obj["input_neuron_ids"] == [0]
    assert json_obj["output_neuron_ids"] == [1]
    assert json_obj["control_neuron_ids"] == []
    assert json_obj["labeled_neurons"] == {}


def test_brain_from_json():
    obj = {
        "neurons": [
            {"id": 0, "bias": 0.1, "reset_factor": 0.2},
            {"id": 1, "bias": -0.1, "reset_factor": 0.3},
        ],
        "edges": [{"source": 0, "target": 1, "weight": 0.5}],
        "input_neuron_ids": [0],
        "control_neuron_ids": [],
        "output_neuron_ids": [1],
    }

    brain = Brain.from_json(obj)

    # neurons loaded
    assert 0 in brain._neurons
    assert 1 in brain._neurons

    n0 = brain._neurons[0]
    n1 = brain._neurons[1]

    assert n0.id == 0
    assert n0.bias == 0.1
    assert n0.reset_factor == 0.2

    assert n1.id == 1
    assert n1.bias == -0.1
    assert n1.reset_factor == 0.3

    # edges loaded
    assert len(brain._edges) == 1
    edge = brain._edges[0]
    assert edge.source == 0
    assert edge.target == 1
    assert edge.weight == 0.5

    assert brain.input_neuron_ids == [0]
    assert brain.control_neuron_ids == []
    assert brain.output_neuron_ids == [1]
