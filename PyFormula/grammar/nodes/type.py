from typing import List


class TypeRefNode:
    def __init__(self, type_name: str, ref_name:str=None):
        # types of same name may belong to different domains like Left.V and Right.V
        # where both left and right are instances of DAGs domain.
        self.type = type_name
        self.ref = ref_name

    def __hash__(self):
        if self.ref:
            return hash(self.type + self.ref)
        else:
            return hash(self.type)


class BasicTypeNode:
    def __init__(self, name, labels, types: List[TypeRefNode]):
        self.name = name
        self.labels = labels
        self.types = types


class UnionTypeNode:
    def __init__(self, name, subtypes):
        self.name = name
        self.subtypes = subtypes
