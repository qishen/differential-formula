from enum import Enum

from executer.term import Term


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

    def get_vars(self):
        return self.term.get_variables()

    def convert(self, pred_type: PredType, negated: bool):
        return Predicate(self.term, pred_type, negated)

    def factset_count(self):
        factset = self.get_factset_for_pred()
        return len(factset)

    def get_factset_for_pred(self, index, optimized=False):
        """
        Return the right factset for predicate constraint, which can be one of
        three fact sets: Existing fact set, Delta fact set and combined fact set.
        :param optimized:
        :return:
        """
        if self.pred_type == PredType.ORIGINAL:
            factset = index.data
        elif self.pred_type == PredType.DELTA:
            if optimized:
                factset = index.optimized_delta_data
            else:
                factset = index.delta_data
        else:
            factset = index.combined_data
        return factset

    def get_factset_for_negated_pred(self, index):
        if not self.negated:
            raise Exception('The constraint has to be a negated predicate.')
        if self.pred_type == PredType.ORIGINAL:
            factset = index.negated_data
        elif self.pred_type == PredType.DELTA:
            factset = index.delta_negated_data
        else:
            factset = index.combined_negated_data
        return factset


class Binary(Constraint):
    def __init__(self):
        pass


class Count(Constraint):
    def __init__(self):
        pass


