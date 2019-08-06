class CompositeTermNode:
    def __init__(self, sort, terms):
        self.type = sort
        self.terms = terms


class VariableTermNode:
    def __init__(self, variable):
        self.variable = variable


class ConstantNode:
    def __init__(self, constant):
        self.constant = constant
