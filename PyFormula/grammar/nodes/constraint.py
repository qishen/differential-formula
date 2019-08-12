class BaseConstraintNode:
    def __init__(self):
        pass


class TermConstraintNode(BaseConstraintNode):
    def __init__(self, negated, term, alias=None):
        self.has_negation = negated
        self.term = term
        self.alias = alias


class BinaryConstraintNode(BaseConstraintNode):
    def __init__(self, left, right, rel_op):
        self.left = left
        self.right = right
        self.op = rel_op


class TypeConstraintNode(BaseConstraintNode):
    def __init__(self, variable, type):
        self.variable = variable
        self.type = type
