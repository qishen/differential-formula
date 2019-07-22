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
    def __init__(self, term: Term, pred_type: PredType=PredType.ORIGINAL, negated: bool=False):
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

    def get_relation(self):
        return self.term.sort

    def convert(self, pred_type: PredType, negated: bool):
        return Predicate(self.term, pred_type, negated)

    def factset_count(self):
        factset = self.get_factset_for_pred()
        return len(factset)

    def get_factset_for_pred(self, optimized=False):
        """
        Return the right factset for predicate constraint, which can be one of
        three fact sets: Existing fact set, Delta fact set and combined fact set.
        :param optimized:
        :return:
        """
        relation = self.get_relation()
        if self.pred_type == PredType.ORIGINAL:
            factset = relation.data
        elif self.pred_type == PredType.DELTA:
            if optimized:
                factset = relation.optimized_delta_data
            else:
                factset = relation.delta_data
        else:
            factset = relation.combined_data
        return factset

    def get_factset_for_negated_pred(self):
        relation = self.get_relation()
        if not self.negated:
            raise Exception('The constraint has to be a negated predicate.')
        if self.pred_type == PredType.ORIGINAL:
            factset = relation.negated_data
        elif self.pred_type == PredType.DELTA:
            factset = relation.delta_negated_data
        else:
            factset = relation.combined_negated_data
        return factset



class Binary(Constraint):
    def __init__(self):
        pass


class Count(Constraint):
    def __init__(self):
        pass


