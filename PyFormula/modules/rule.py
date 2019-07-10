from modules.constraint import Constraint, PredType, Predicate
from typing import *


class Rule:
    def __init__(self, head: List[Constraint], body: List[Constraint]):
        self.head = head
        self.body = body

    def __str__(self):
        return ', '.join([str(pred) for pred in self.head]) + ' :- ' + ', '.join([str(pred) for pred in self.body])

    def derive_delta_rules(self):
        rules = []
        length = len(self.body)
        for i in range(length):
            body = []
            for m in range(0, i):
                body.append(self.body[m].convert(PredType.COMBINED, False))
            body.append(self.body[i].convert(PredType.DELTA, False))
            for n in range(i+1, length):
                body.append(self.body[n].convert(PredType.ORIGINAL, False))

            head = []
            for pred in self.head:
                head.append(pred.convert(PredType.DELTA, False))

            new_rule = Rule(head, body)
            rules.append(new_rule)

        return rules

    def get_factset_for_pred(self, constraint):
        """
        Return the right factset for predicate constraint, which can be one of
        three fact sets: Existing fact set, Delta fact set and combined fact set.
        :param constraint:
        :return:
        """
        relation = constraint.get_relation()
        if constraint.pred_type == PredType.ORIGINAL:
            factset = relation.data
        elif constraint.pred_type == PredType.DELTA:
            factset = relation.delta_data
        else:
            factset = relation.combined_data
        return factset

    def find_match(self):
        bindings_with_count_list = [({}, 1)]
        for constraint in self.body:
            factset = self.get_factset_for_pred(constraint)
            new_bindings_with_count_list = []
            for fact in factset:
                fact_count = factset[fact]
                for bindings_tuple in bindings_with_count_list:
                    (bindings, bindings_count) = bindings_tuple
                    partial_binded_term = constraint.term.propagate_bindings(bindings)
                    '''
                    If the term in constraint predicate is still not fully binded after propagating bindings
                    and the partial binded term is semantically equal to current ground term fact, then find
                    new bindings between partial binded term and fact.
                    '''
                    if partial_binded_term.is_ground_term:
                        new_bindings_with_count_list.append((bindings, bindings_count * fact_count))
                    elif not partial_binded_term.is_ground_term and partial_binded_term.equal_semantically(fact):
                        new_bindings = partial_binded_term.get_bindings(fact)
                        new_combined_bindings = {**bindings, **new_bindings}
                        new_combined_bindings_count = fact_count * bindings_count
                        new_combined_bindings_tuple = (new_combined_bindings, new_combined_bindings_count)
                        new_bindings_with_count_list.append(new_combined_bindings_tuple)
            bindings_with_count_list = new_bindings_with_count_list
        return bindings_with_count_list


if __name__ == '__main__':
    pass