from .ddengine import DDExecuter, Atom, Variable, Composite, BasicType, BuiltInType

__all__ = ["DDExecuter", "Atom", "Variable", "Composite", "BasicType", "BuiltInType"]


def search_py(path, needle):
    total = 0
    with open(path, "r") as f:
        for line in f:
            words = line.split(" ")
            for word in words:
                if word == needle:
                    total += 1
    return total