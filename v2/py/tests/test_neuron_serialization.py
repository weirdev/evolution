import sys
import pathlib

# Add src to sys.path so `evolution` package is importable
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1] / "src"))

from evolution.neuron import Neuron


def test_neuron_to_json():
    neuron = Neuron(id=42, bias=0.25, reset_factor=0.8)
    json_obj = neuron.to_json()

    assert isinstance(json_obj, dict)
    assert json_obj == {
        "id": 42,
        "bias": 0.25,
        "reset_factor": 0.8,
    }


def test_neuron_from_json():
    obj = {"id": 7, "bias": -0.5, "reset_factor": 0.25}
    neuron = Neuron.from_json(obj)

    assert neuron.id == 7
    assert neuron.bias == -0.5
    assert neuron.reset_factor == 0.25
    # activation should be at default 0.0
    assert neuron.activation == 0.0
