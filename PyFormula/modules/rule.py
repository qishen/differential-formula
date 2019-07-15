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

    def find_match(self):
        bindings_with_count_list = [({}, 1)]
        negated_constraints = []
        for constraint in self.body:
            if constraint.negated:
                negated_constraints.append(constraint)
            else:
                factset = constraint.get_factset_for_pred()
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

        ''' Get all feasible bindings from non-negated terms and filter them according to matches on negated terms '''
        for negated_constraint in negated_constraints:
            negated_term = negated_constraint.term
            for bindings_with_count in bindings_with_count_list:
                (bindings, count) = bindings_with_count
                binded_negated_term = negated_term.propagate_bindings(bindings)
                '''
                It is possible that negated term contains variables that don't exist in bindings. 
                TODO: Compiler should do some sanity checks to make sure variables in negated term always occur
                in other sub-goals under same rule.
                '''
                if binded_negated_term.is_ground_term:
                    if negated_constraint.pred_type == PredType.DELTA:
                        if binded_negated_term in negated_term.sort.delta_data and binded_negated_term not in negated_term.sort.combined_data:
                            negated_term.sort.delta_negated_data[binded_negated_term] = 1
                        elif binded_negated_term in negated_term.sort.delta_data and binded_negated_term not in negated_term.sort.data:
                            negated_term.sort.delta_negated_data[binded_negated_term] = -1
                            bindings_with_count_list.remove(bindings_with_count)
                            bindings_with_count_list.append((bindings, count*-1))
                    else:
                        if negated_constraint.pred_type == PredType.ORIGINAL:
                            un_negated_terms = negated_term.sort.data
                        else:  # PredType.COMBINED
                            un_negated_terms = negated_term.sort.combined_data

                        if binded_negated_term not in un_negated_terms:
                            negated_term.sort.negated_data[binded_negated_term] = 1
                        else:
                            # Remove the binding if negated term is not satisfied.
                            bindings_with_count_list.remove(bindings_with_count)

        return bindings_with_count_list


if __name__ == '__main__':
    pass
