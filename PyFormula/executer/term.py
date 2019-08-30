from enum import Enum

from executer.relation import *


# A term can be atom, variable or composition of terms
class TermType(Enum):
    ATOM = 1
    VARIABLE = 2
    COMPOSITE = 3


class Term:
    def __init__(self, sort):
        self.sort = sort

    def get_variables(self):
        variables = []
        if self.check_ground_term():
            return variables
        else:
            if type(self) is Variable:
                variables.append(self)
            elif type(self) is Composite:
                for term in self.args:
                    sub_vars = term.get_variables()
                    variables += sub_vars
        return variables

    def equal_semantically(self, other):
        """
        Two terms are semantically equivalent if and only if:
        1. one of them or both are variables to represent instances of same type.
        2. two atom terms have same value
        3. two composite terms have same type and their argument terms are also semantically equivalent.
        :param other:
        :return:
        """
        if (type(self) is Variable or type(other) is Variable) and (self.sort == other.sort):
            return True
        elif type(self) is Atom and type(other) is Atom:
            return self.val == other.val
        elif type(self) is Composite and type(other) is Composite:
            if self.relation == other.relation:
                length = len(self.args)
                for i in range(length):
                    if not self.args[i].equal_semantically(other.args[i]):
                        return False
                return True
            else:
                return False
        else:
            # Atom vs Composite is definitely false.
            return False

    def get_bindings(self, ground_term):
        """
        Self must be a non-ground term and the argument has to be ground term.
        Assume both terms are from same relation or sort.
        A composite term with variables is compared with a semantically equivalent ground term
        to get bindings.
        :param ground_term:
        :return:
        """
        if self.is_ground_term or not ground_term.is_ground_term:
            raise Exception('Self is ground term or self is compared to a non-ground term.')

        def bind_helper(term, gterm, bindings):
            """
            Recursively get bindings without checking equality semantically.
            :param term:
            :param gterm:
            :param bindings:
            :return:
            """
            if type(term) is Variable:
                bindings[term] = gterm
            elif type(term) is Atom:
                if term.val != gterm.val:
                    bindings.clear()
                    return False
            elif type(term) is Composite:
                length = len(term.args)
                for i in range(length):
                    has_local_bindings = bind_helper(term.args[i], gterm.args[i], bindings)
                    if not has_local_bindings:
                        return False
            return True

        all_bindings = {}
        has_bindings = bind_helper(self, ground_term, all_bindings)
        if has_bindings:
            return all_bindings
        else:
            return {}


class Composite(Term):
    def __init__(self, relation, terms, alias=None, domain_ref=None):
        super().__init__(relation)
        self.relation = relation  # same as self.sort attribute in base class.
        self.args = terms
        self.alias = alias
        self.domain_ref = domain_ref # in::Graph and out::Graph
        self.term_type = TermType.COMPOSITE
        self.is_ground_term = self.check_ground_term()

    def __str__(self):
        return self.relation.name + '(' + ','.join([str(x) for x in self.args]) + ')'

    def __hash__(self):
        hashable_list = [arg.__hash__() for arg in self.args] + [self.relation.name]
        return hash(tuple(hashable_list))

    def __eq__(self, other):
        return self.__hash__() == other.__hash__()

    def propagate_bindings(self, bindings):
        length = len(self.args)
        terms = []
        for i in range(length):
            terms.append(self.args[i].propagate_bindings(bindings))
        return Composite(self.relation, terms)

    def replace_variables_in_place(self, alias_to_fact_map):
        def _replace_variables_in_place_helper(term, alias_to_fact_map):
            for i, subterm in enumerate(term.args):
                if type(subterm) is Variable:
                    var_name = subterm.var
                    if var_name in alias_to_fact_map:
                        term.args[i] = alias_to_fact_map[var_name]
                elif type(subterm) is Composite:
                    _replace_variables_in_place_helper(subterm, alias_to_fact_map)

        # Need to update ground term status after variable replacement.
        _replace_variables_in_place_helper(self, alias_to_fact_map)
        self.is_ground_term = self.check_ground_term()

    def check_ground_term(self):
        for arg in self.args:
            if not arg.check_ground_term():
                return False
        return True


class Atom(Term):
    def __init__(self, value):
        # Determine the sort of basic atom value.
        if type(value) is str:
            sort = BuiltInType('String')
        elif type(value) is int:
            sort = BuiltInType('Integer')
        else:
            sort = BuiltInType('Float')

        super().__init__(sort)
        self.val = value
        self.term_type = TermType.ATOM

    def __str__(self):
        if type(self.val) is str:
            return '"' + self.val + '"'
        else:
            return str(self.val)

    def __hash__(self):
        return hash(self.val)

    def __eq__(self, other):
        if type(other) == Atom:
            return self.val == other.val
        else:
            return False

    def propagate_bindings(self, bindings):
        return self

    def check_ground_term(self):
        return True


class Variable(Term):
    def __init__(self, name: str, sort: BasicType):
        super().__init__(sort)
        self.var = name.split('.')[0]
        self.term_type = TermType.VARIABLE
        self.fragments = name.split('.')[1:]

    def __str__(self):
        return '<' + self.var + '>'

    def __hash__(self):
        return hash(self.var)

    def __eq__(self, other):
        if type(other) == Variable:
            return self.var == other.var
        else:
            return False

    def propagate_bindings(self, bindings):
        # Don't have to deep clone for a new term because ground term is immutable.
        if self in bindings:
            return bindings[self]
        else:
            return self

    def check_ground_term(self):
        return False


