import json
from os import PathLike, makedirs

type JsonPrimitive = int | float | str
type JsonArray = list["JsonValue"]
type JsonValue = "JsonPrimitive" | "JsonArray" | "JsonObject"
type JsonObject = dict[str, "JsonValue"]


def write_to_file(filename: PathLike, obj: JsonObject):
    with open(filename, "x") as f:
        json.dump(obj, f)


def read_from_file(filename: PathLike) -> JsonObject:
    with open(filename) as f:
        return json.load(f)
