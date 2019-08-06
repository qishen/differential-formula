class BaseConstraintNode:
    def __init__(self):
        pass


class TermConstraintNode(BaseConstraintNode):
    def __init__(self, negated, term, alias=None):
        self.has_negation = negated
        self.term = term
        self.alias = alias
