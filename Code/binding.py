import ctypes
from dataclasses import dataclass
from typing import List
__all__ = ["Command", "request"]
@dataclass
class Command:
    func: bool
    name: str
    value: str

l = ctypes.CDLL("./gset.dll")
l.Request.argtypes = (ctypes.c_char_p,)
l.Request.restype = ctypes.c_char_p

def request(url: str) -> List[Command]:
    result = l.Request("https://raw.githubusercontent.com/Kolya142/my-prog-settings/main/test.sett".encode("utf-8"))
    result = ctypes.string_at(result).decode('utf-8')
    commands = []
    for line in result.split("\n"):
        split = line.split(";")
        if len(split) != 3:
            continue
        commands.append(Command(split[0] == "1", split[1], split[2]))
    return commands
