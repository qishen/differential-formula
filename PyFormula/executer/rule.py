from collections import Counter
from typing import *

from executer.constraint import Pattern, BaseConstraint, PredType


class Rule(Pattern):
    def __init__(self, head: List[BaseConstraint], body: List[List[BaseConstraint]]):
        super().__init__(body)
        self.head = head
        self.has_recursion = self.check_recursion()

    def __str__(self):
        return ', '.join([str(pred) for pred in self.head]) + ' :- ' + super.__str__()

    def check_recursion(self):
        for conjunction in self.body:
            for body_constraint in conjunction:
                for head_constraint in self.head:
                    if body_constraint.term.sort.name == head_constraint.term.sort.name:
                        return True
        return False

    def derive_delta_rules(self):
        """
        Derive a set of delta rules that each rule has only one delta predicate on every possible
        occurrence, predicates before delta pred are all PredType.COMBINED while preds after delta
        pred are all PredType.ORIGINAL
        :return:
        """
        rules = []
        length = len(self.body)
        for i in range(length):
            body = []
            for m in range(0, i):
                negated = self.body[m].negated
                body.append(self.body[m].convert(PredType.COMBINED, negated))

            negated = self.body[i].negated
            body.append(self.body[i].convert(PredType.DELTA, negated))

            for n in range(i+1, length):
                negated = self.body[n].negated
                body.append(self.body[n].convert(PredType.ORIGINAL, negated))

            # Head cannot be negated and has to be positive
            head = []
            for pred in self.head:
                head.append(pred.convert(PredType.DELTA, False))

            new_rule = Rule(head, body)
            rules.append(new_rule)

        return rules
