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
        bindings_with_count_list = [[{}, 1]]
        negated_constraints = []
        term_constraints = []

        for constraint in self.body:
            if constraint.negated:
                negated_constraints.append(constraint)
            else:
                term_constraints.append(constraint)

        ''' Sort term constraints by facts number '''
        term_constraints.sort(key=lambda x: x.factset_count())

        '''
        Find all bindings for term constraints in the body excluding all negated constraints but put them in a list.
        '''
        for constraint in term_constraints:
            ''' Can be either original, delta or combined fact set data depending on constraint prefix.'''
            factset = constraint.get_factset_for_pred()
            new_bindings_with_count_list = []

            ''' No bindings since factset is empty, return [] immediately '''
            if len(factset) == 0:
                return []

            for fact in factset:
                fact_count = factset[fact]
                for bindings_tuple in bindings_with_count_list:
                    [bindings, bindings_count] = bindings_tuple
                    partial_binded_term = constraint.term.propagate_bindings(bindings)
                    '''
                    If the term in constraint predicate is still not fully binded after propagating bindings
                    and the partial binded term is semantically equal to current ground term fact, then find
                    new bindings between partial binded term and fact.
                    '''
                    if partial_binded_term.is_ground_term:
                        new_bindings_with_count_list.append([bindings, bindings_count * fact_count])
                    else:
                        new_bindings = partial_binded_term.get_bindings(fact)
                        if len(new_bindings) > 0:
                            new_combined_bindings = {**bindings, **new_bindings}
                            new_combined_bindings_count = fact_count * bindings_count
                            new_combined_bindings_tuple = [new_combined_bindings, new_combined_bindings_count]
                            new_bindings_with_count_list.append(new_combined_bindings_tuple)
            bindings_with_count_list = new_bindings_with_count_list

        ''' 
        Get all feasible bindings from non-negated terms and filter them according to matches on negated terms data
        '''
        for negated_constraint in negated_constraints:
            delete_bindings_with_count_list = []
            negated_term = negated_constraint.term
            for index, bindings_with_count in enumerate(bindings_with_count_list):
                [bindings, count] = bindings_with_count
                binded_negated_term = negated_term.propagate_bindings(bindings)
                '''
                It is possible that negated term contains variables that don't exist in bindings. 
                TODO: Compiler should do some sanity checks to make sure variables in negated term always occur
                in other sub-goals under same rule.
                '''
                if binded_negated_term.is_ground_term:
                    if negated_constraint.pred_type == PredType.DELTA:
                        '''
                        1. t in delta Q and t not in Q + delta Q --> count = 1
                        2. t in delta Q and t not in Q --> count = -1
                        3. delta Q is empty, simply remove the binding tuple.
                        '''
                        if binded_negated_term in negated_term.sort.delta_data and binded_negated_term not in negated_term.sort.combined_data:
                            negated_term.sort.delta_negated_data[binded_negated_term] = 1
                        elif binded_negated_term in negated_term.sort.delta_data and binded_negated_term not in negated_term.sort.data:
                            negated_term.sort.delta_negated_data[binded_negated_term] = -1
                            ''' Update bindings count regarding negated constraint and 
                                there can be only one delta negated constraint in rule'''
                            new_count = count * -1
                            bindings_with_count_list[index][1] = new_count
                        else:
                            ''' Delta negated constraint is not satisfied and need to remove the bindings. '''
                            delete_bindings_with_count_list.append(bindings_with_count)
                    else:
                        if negated_constraint.pred_type == PredType.ORIGINAL:
                            terms = negated_term.sort.data
                        elif negated_constraint.pred_type == PredType.COMBINED:
                            terms = negated_term.sort.combined_data
                        ''' 
                        Remove the binding if negated term is not satisfied.
                        '''
                        if binded_negated_term not in terms:
                            negated_term.sort.negated_data[binded_negated_term] = 1
                        else:
                            delete_bindings_with_count_list.append(bindings_with_count)

            ''' Delete some bindings that don't satisfy negated constraint terms.'''
            for delete_bindings_with_count in delete_bindings_with_count_list:
                bindings_with_count_list.remove(delete_bindings_with_count)

        return bindings_with_count_list


if __name__ == '__main__':
    pass
