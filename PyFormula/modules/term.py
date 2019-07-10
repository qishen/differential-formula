from enum import Enum
from typing import *
from modules.relation import Relation
from deepdiff import DeepHash


# A term can be atom, variable or composition of terms
class TermType(Enum):
    ATOM = 1
    VARIABLE = 2
    COMPOSITE = 3


class Term:
    def __init__(self, sort):
        self.sort = sort

    def check_ground_term(self):
        if type(self) is Atom:
            return True
        elif type(self) is Variable:
            return False
        elif type(self) is Composite:
            for arg in self.args:
                if not arg.check_ground_term():
                    return False
            return True

    def get_variables(self):
        variables = []
        if self.check_ground_term():
            return variables
        else:
            if type(self) is Variable:
                variables.append(self)
            elif type(self) is Composite:
                for term in self.args:
                    sub_vars = self.get_variables(term)
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

    def equal_strictly(self, other):
        """
        Two terms are strictly equivalent if they are exactly the same as well as their argument terms.
        :param other:
        :return:
        """
        if (type(self) is Variable and type(other) is Variable) and (self.sort == other.sort):
            return True
        elif type(self) is Atom and type(other) is Atom:
            return self.val == other.val
        elif type(self) is Composite and type(other) is Composite:
            if self.relation == other.relation:
                length = len(self.args)
                for i in range(length):
                    if not self.args[i].equal_strictly(other.args[i]):
                        return False
                    return True
            else:
                return False
        else:
            return False

    def get_bindings(self, ground_term):
        """
        TODO: This method can be optimized to reduce redundant sanity check.
        A composite term with variables is compared with a semantically equivalent ground term
        to get bindings.
        :param ground_term:
        :return:
        """
        bindings = {}

        def bind_helper(var_term, g_term):
            """
            Recursively bind variables to ground terms without invoking all sanity checks again.
            :param var_term:
            :param g_term:
            :return:
            """
            if type(var_term) is Variable:
                bindings[var_term] = g_term
            elif type(var_term) is Composite:
                length = len(var_term.args)
                for i in range(length):
                    bind_helper(var_term.args[i], g_term.args[i])

        if not self.equal_semantically(ground_term):
            raise Exception('Two terms are not semantically equal and thus cannot be bind.')
        elif not ground_term.is_ground_term:
            raise Exception('Cannot bind to non-ground term.')
        else:
            bind_helper(self, ground_term)
        return bindings

    def propagate_bindings(self, bindings):
        """
        Propagate variable bindings to a composite term that contains variables,
        return a new term with some or all variables substituted by bindings as result,
        or just return the original immutable variable or atom term.
        :param bindings:
        :return:
        """
        if type(self) == Variable:
            for (v, t) in bindings.items():
                # Don't have to deep clone for a new term because ground term is immutable.
                if self == v:
                    return bindings[v]
            # Do not have suitable binding for this variable, so just return the variable.
            return self
        elif type(self) == Atom:
            return self
        elif type(self) == Composite:
            length = len(self.args)
            terms = []
            for i in range(length):
                terms.append(self.args[i].propagate_bindings(bindings))
            return Composite(self.relation, terms)


class Composite(Term):
    def __init__(self, relation, terms, alias=None):
        super().__init__(relation)
        self.relation = relation  # same as self.sort attribute in base class.
        self.args = terms
        self.alias = alias
        self.term_type = TermType.COMPOSITE
        self.is_ground_term = self.check_ground_term()

        # Can't call DeepHash on self because it will invoke __hash__() and end up with endless recursion.
        hash_obj = [self.args, self.relation.name]
        self.hash = hash(DeepHash(hash_obj)[hash_obj])


    def __str__(self):
        return self.relation.name + '(' + ','.join([str(x) for x in self.args]) + ')'

    def __hash__(self):
        return self.hash

    def __eq__(self, other):
        return self.__hash__() == other.__hash__()


class Atom(Term):
    def __init__(self, value):
        # Determine the sort of basic atom value.
        if type(value) is str:
            sort = Relation('string')
        elif type(value) is int:
            sort = Relation('integer')
        else:
            sort = Relation('float')

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
        return self.__hash__() == other.__hash__()

class Variable(Term):
    def __init__(self, name: str, sort: Relation):
        super().__init__(sort)
        self.var = name
        self.term_type = TermType.VARIABLE

    def __str__(self):
        return '<' + self.var + '>'

    def __hash__(self):
        return hash(self.var)

    def __eq__(self, other):
        return self.__hash__() == other.__hash__()


if __name__ == '__main__':
    node = Relation('node', ['id'], ['string'])
    edge = Relation('edge', ['src', 'dst'], ['node', 'node'])
    a1 = Atom('hello')
    a2 = Atom(123)
    a3 = Atom(1.23)
    a4 = Atom('world')

    v1 = Variable('X', node)
    v2 = Variable('Y', node)
    s1 = Variable('S', Relation('string'))

    n1 = Composite(node, [a1])
    n1_clone = Composite(node, [a1])
    n2 = Composite(node, [s1])
    n3 = Composite(node, [a4])

    e1 = Composite(edge, [v1, v2])
    e2 = Composite(edge, [n1, v2])
    e3 = Composite(edge, [n1, n3])
    e3_clone = Composite(edge, [n1, n3])

    link = Relation('link', ['src', 'dst'], ["string", "string"])
    string_sort = Relation('string')
    link_x_z_term = Composite(link, [Variable('X', string_sort), Variable('Z', string_sort)])
    bindings = {Variable('X', string_sort): Atom('hello'),Variable('Z', string_sort): Atom('world')}

    print(a1, a2, a3)
    print(v1, v2)
    print(n1, n2)
    print(e1, e2)

    print(n1.hash)
    print(n1_clone.hash)
    print(n2.hash)
    print(e3.hash)
    print(e3_clone.hash)

    t = link_x_z_term.propagate_bindings(bindings)
    print(t)