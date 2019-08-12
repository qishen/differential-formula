class BasicTypeNode:
    def __init__(self, name, labels, types):
        self.name = name
        self.labels = labels
        self.types = types


class UnionTypeNode:
    def __init__(self, name, subtypes):
        self.name = name
        self.subtypes = subtypes
