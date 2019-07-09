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

    def find_match(self):
        pass
