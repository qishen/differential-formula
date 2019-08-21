from grammar.nodes.type import TypeRefNode


class CompositeTermNode:
    def __init__(self, type_ref: TypeRefNode, terms):
        self.type = type_ref
        self.terms = terms


class VariableTermNode:
    def __init__(self, variable):
        self.variable = variable


class ConstantNode:
    def __init__(self, constant):
        self.constant = constant
