from enum import Enum
from typing import *

from executer.binding import Bindings, BindingsCounter
from executer.term import Term


class PredType(Enum):
    ORIGINAL = 1
    DELTA = 2
    COMBINED = 3


class BaseConstraint:
    def __init__(self):
        pass


class Predicate(BaseConstraint):
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


class Pattern(BaseConstraint):
    def __init__(self, body: List[List[BaseConstraint]]):
        self.body = body
        self.negated_constraints_list = []
        self.term_constraints_list = []

        for conjunction in self.body:
            negated_constraints = []
            term_constraints = []
            for constraint in conjunction:
                if constraint.negated:
                    negated_constraints.append(constraint)
                else:
                    term_constraints.append(constraint)
            self.negated_constraints_list.append(negated_constraints)
            self.term_constraints_list.append(term_constraints)

        # TODO: Try to find an optimal constraint execution order.
        self.optimize_constraints_order()

    def __str__(self):
        conjunction_strs = []
        for conjunction in self.body:
            conjunction_str = ','.join([str(pred) for pred in conjunction])
            conjunction_strs.append(conjunction_str)
        return ';'.join(conjunction_strs)

    def optimize_constraints_order(self):
        """
        Start with the term with largest number of vars, then pick up the next term that has the largest
        number of intersection compared with all variables found in previous terms.
        :return:
        """
        optimized_constraints_list = []
        for term_constraints in self.term_constraints_list:
            optimized_constraints = []
            all_vars = set()
            term_constraints.sort(key=lambda x: len(x.get_vars()), reverse=True)
            c = term_constraints.pop(0)
            all_vars.update(c.get_vars())
            optimized_constraints.append(c)
            while len(term_constraints) > 0:
                term_constraints.sort(key=lambda x: len(set(x.get_vars()).intersection(all_vars)), reverse=True)
                c = term_constraints.pop(0)
                optimized_constraints.append(c)
                all_vars.update(c.get_vars())
            optimized_constraints_list.append(optimized_constraints)
        self.term_constraints_list = optimized_constraints_list

    def find_match_without_counting(self):
        """
        Implementatino of DRed Algorithm for semi-naive rule evaluation
        :return:
        """
        bindings_counter = BindingsCounter({Bindings(): 1})
        for constraint in self.term_constraints:
            factset = constraint.get_factset_for_pred()
            new_bindings_counter = BindingsCounter()

            if len(factset) == 0:
                return BindingsCounter()

            for bindings in bindings_counter:
                partial_binded_term = constraint.term.propagate_bindings(bindings)
                if partial_binded_term.is_ground_term:
                    if partial_binded_term in factset:
                        new_bindings_counter.update({bindings: 1})
                else:
                    for fact in factset:
                        new_bindings = Bindings(partial_binded_term.get_bindings(fact))
                        if len(new_bindings) > 0:
                            new_bindings.update(bindings)
                            new_bindings_counter.update({new_bindings: 1})
            bindings_counter = new_bindings_counter

        for negated_constraint in self.negated_constraints:
            delete_bindings_list = []
            negated_term = negated_constraint.term
            for bindings in bindings_counter:
                binded_negated_term = negated_term.propagate_bindings(bindings)
                if binded_negated_term.is_ground_term:
                    pass

            ''' Delete some bindings that don't satisfy negated constraint terms.'''
            for delete_bindings in delete_bindings_list:
                del bindings_counter[delete_bindings]

        return bindings_counter

    def find_match(self, type_index_map):
        """
        Implementation of Counting Algorithm for rule execution
        :return:
        """
        bindings_counter = BindingsCounter({Bindings(): 1})
        '''
        Find all bindings for term constraints in the body excluding all negated constraints but put them in a list.
        '''
        for constraint in self.term_constraints_list[0]:
            index = type_index_map[constraint.term.sort.name]
            ''' Can be either original, delta or combined fact set data depending on constraint prefix. '''
            factset = constraint.get_factset_for_pred(index)
            new_bindings_counter = BindingsCounter()

            ''' No bindings since factset is empty, return [] immediately '''
            # TODO: implement early termination if one of the constraint has an empty data set.
            if len(factset) == 0:
                return BindingsCounter()

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
        for negated_constraint in self.negated_constraints_list[0]:
            delete_bindings_list = []
            negated_term = negated_constraint.term
            index = type_index_map[negated_term.sort.name]

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
                        if binded_negated_term in index.delta_data and binded_negated_term not in index.combined_data:
                            index.delta_negated_data[binded_negated_term] = 1
                        elif binded_negated_term in index.delta_data and binded_negated_term not in index.data:
                            index.delta_negated_data[binded_negated_term] = -1
                            ''' Update bindings count regarding negated constraint and 
                                there can be only one delta negated constraint in rule'''
                            new_count = count * -1
                            bindings_counter[bindings] = new_count
                        else:
                            ''' Delta negated constraint is not satisfied and need to remove the bindings. '''
                            delete_bindings_list.append(bindings)
                    else:
                        if negated_constraint.pred_type == PredType.ORIGINAL:
                            terms = index.data
                        elif negated_constraint.pred_type == PredType.COMBINED:
                            terms = index.combined_data
                        ''' 
                        Remove the binding if negated term is not satisfied.
                        '''
                        if binded_negated_term not in terms:
                            index.negated_data[binded_negated_term] = 1
                        else:
                            delete_bindings_list.append(bindings)

            ''' Delete some bindings that don't satisfy negated constraint terms.'''
            for delete_bindings in delete_bindings_list:
                del bindings_counter[delete_bindings]

        return bindings_counter


class Expression:
    def __init__(self):
        pass


class SetComprehension(Expression):
    def __init__(self, head_terms, constraints):
        self.head_terms = head_terms
        self.constraints = constraints

    def evaluate(self):
        pass


class Aggregation(Expression):
    def __init__(self, func_name, set_comprehension, tid=None, default_value=None):
        self.func = func_name
        self.set_comprehension = set_comprehension
        self.tid = tid
        self.default_value = default_value

    def evaluate(self):
        pass


class BinaryConstraint(BaseConstraint):
    def __init__(self, left, right, op):
        self.left = left
        self.right = right
        self.op = op

    def evaluate(self, var_map):
        pass


class TypeConstraint(BaseConstraint):
    def __init__(self):
        pass
