import json
import re


def decode_red(dct):
    for value in dct.values():
        if not isinstance(value, list) and value == "red":
            return "{}"
    return dct


json_object = json.load(open(f'./12.txt', "r"),  object_hook=decode_red)
numbers = re.sub(r'[^\d-]+', "+", json.dumps(json_object))
print(eval(numbers + "0"))
