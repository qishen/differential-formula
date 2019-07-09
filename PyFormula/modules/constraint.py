from enum import Enum
from typing import *

from modules.term import Term, Atom, Variable, Composite
from modules.relation import Relation


class PredType(Enum):
    ORIGINAL = 1
    DELTA = 2
    COMBINED = 3


class Constraint:
    def __init__(self):
        pass


class Predicate(Constraint):
    def __init__(self, term: Term, pred_type: PredType, negated: bool):
        self.term = term
        self.pred_type = pred_type
        self.negated = negated

    def __str__(self):
        prefix = ''
        if self.negated:
            prefix += 'no '
        if self.pred_type == PredType.DELTA:
            prefix += '[delta]'
        elif self.pred_type == PredType.COMBINED:
            prefix += '[combined]'
        return prefix + str(self.term)

    def convert(self, pred_type: PredType, negated: bool):
        return Predicate(self.term, pred_type, negated)



class Binary(Constraint):
    def __init__(self):
        pass


class Count(Constraint):
    def __init__(self):
        pass


