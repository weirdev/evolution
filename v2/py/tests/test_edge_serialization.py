import sys
import pathlib

# Add src to sys.path so `evolution` package is importable
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1] / "src"))

from evolution.neuron import Edge


def test_edge_to_json():
    edge = Edge(source=1, target=2, weight=0.5)
    json_obj = edge.to_json()

    assert isinstance(json_obj, dict)
    assert json_obj == {"source": 1, "target": 2, "weight": 0.5}


def test_edge_from_json():
    obj = {"source": 3, "target": 4, "weight": -0.75}
    edge = Edge.from_json(obj)

    assert edge.source == 3
    assert edge.target == 4
    assert edge.weight == -0.75
