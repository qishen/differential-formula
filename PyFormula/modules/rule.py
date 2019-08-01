from typing import *
from collections import Counter

from modules.constraint import Constraint, PredType, Predicate


class Bindings(dict):
    """
    Inherit built-in dict class with hash value computed on both keys and values,
    while default dict is not hashable.
    """
    def __init__(self, initial_bindings={}):
        self.update(initial_bindings)

    def _members(self):
        return tuple(list(self.keys()) + list(self.values()))

    def __eq__(self, other):
        if len(self) is not len(other):
            return False
        else:
            for key in self:
                if key not in other or self[key] != other[key]:
                    return False
            return True

    def __hash__(self):
        return hash(self._members())

    def __str__(self):
        bindings_str_list = []
        for (key, value) in self.items():
            bindings_str_list.append('[' + str(key) + ' binds to ' + str(value) + ']')
        return ', '.join(bindings_str_list)


class BindingsCounter(Counter):
    """
    Use Counter to remove possible duplicates when some different bindings are extended
    and results in the same new bindings
    """
    def __str__(self):
        bindings_counter_str = ''
        for bindings in self:
            bindings_counter_str += str(bindings) + ' with count ' + str(self[bindings]) + '\n'
        return bindings_counter_str


class Rule:
    def __init__(self, head: List[Constraint], body: List[Constraint]):
        self.head = head
        self.body = body
        self.has_recursion = self.check_recursion()
        self.negated_constraints = []
        self.term_constraints = []

        for constraint in self.body:
            if constraint.negated:
                self.negated_constraints.append(constraint)
            else:
                self.term_constraints.append(constraint)

        self.optimize_constraints_order()

    def __str__(self):
        return ', '.join([str(pred) for pred in self.head]) + ' :- ' + ', '.join([str(pred) for pred in self.body])

    def check_recursion(self):
        for body_constraint in self.body:
            for head_constraint in self.head:
                if body_constraint.term.sort.name == head_constraint.term.sort.name:
                    return True
        return False

    def optimize_constraints_order(self):
        """
        Start with the term with largest number of vars, then pick up the next term that has the largest
        number of intersection compared with all variables found in previous terms.
        :return:
        """
        optimized_constraints = []
        all_vars = set()
        self.term_constraints.sort(key=lambda x: len(x.get_vars()), reverse=True)
        c = self.term_constraints.pop(0)
        all_vars.update(c.get_vars())
        optimized_constraints.append(c)
        while len(self.term_constraints) > 0:
            self.term_constraints.sort(key=lambda x: len(set(x.get_vars()).intersection(all_vars)), reverse=True)
            c = self.term_constraints.pop(0)
            optimized_constraints.append(c)
            all_vars.update(c.get_vars())
        self.term_constraints = optimized_constraints

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

    def find_match_without_counting(self):
        bindings_counter = BindingsCounter({Bindings(): 1})
        for constraint in self.term_constraints:
            factset = constraint.get_factset_for_pred()

    def find_match(self):
        bindings_counter = BindingsCounter({Bindings(): 1})
        '''
        Find all bindings for term constraints in the body excluding all negated constraints but put them in a list.
        '''
        for constraint in self.term_constraints:
            ''' Can be either original, delta or combined fact set data depending on constraint prefix. '''
            factset = constraint.get_factset_for_pred()
            new_bindings_counter = BindingsCounter()

            ''' No bindings since factset is empty, return [] immediately '''
            if len(factset) == 0:
                return []

            for bindings in bindings_counter:
                bindings_count = bindings_counter[bindings]
                partial_binded_term = constraint.term.propagate_bindings(bindings)
                '''
                1. If the term in constraint predicate is still not fully binded after propagating bindings
                and the partial binded term is semantically equal to current ground term fact, then find
                new bindings between partial binded term and fact.
                2. Ground term after propagation, then check if that term exists before adding the bindings.
                '''
                if partial_binded_term.is_ground_term:
                    if partial_binded_term in factset:
                        new_bindings_counter.update({bindings: bindings_count})
                else:
                    for fact in factset:
                        fact_count = factset[fact]
                        new_bindings = Bindings(partial_binded_term.get_bindings(fact))
                        if len(new_bindings) > 0:
                            new_bindings.update(bindings)
                            new_combined_bindings_count = fact_count * bindings_count
                            new_bindings_counter.update({new_bindings: new_combined_bindings_count})
            bindings_counter = new_bindings_counter

        ''' 
        Get all feasible bindings from non-negated terms and filter them according to matches on negated terms data
        '''
        for negated_constraint in self.negated_constraints:
            delete_bindings_list = []
            negated_term = negated_constraint.term
            for bindings in bindings_counter:
                count = bindings_counter[bindings]
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
                            bindings_counter[bindings] = new_count
                        else:
                            ''' Delta negated constraint is not satisfied and need to remove the bindings. '''
                            delete_bindings_list.append(bindings)
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
                            delete_bindings_list.append(bindings)

            ''' Delete some bindings that don't satisfy negated constraint terms.'''
            for delete_bindings in delete_bindings_list:
                del bindings_counter[delete_bindings]

        return bindings_counter
