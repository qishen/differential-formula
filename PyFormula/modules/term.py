from enum import Enum


class TermType(Enum):
    ATOM = 1
    VARIABLE = 2
    COMPOSITE = 3


'''
A term can be atom, variable or composition of terms
'''
class Term:
    def __init__(self):
        pass

    def is_ground_term(self):
        if type(self) is Atom:
            return True
        elif type(self) is Variable:
            return False
        elif type(self) is Composite:
            for arg in self.args:
                if not arg.is_ground_term():
                    return False
            return True


    def get_variables(self):
        variables = []
        if self.is_ground_term():
            return variables
        else:
            if type(self) is Variable:
                variables.append(self)
            elif type(self) is Composite:
                for term in self.args:
                    sub_vars = self.get_variables(term)
                    variables += sub_vars
        return variables


    @staticmethod
    def get_bindings(term, ground_term):
        bindings = {}





class Composite(Term):
    def __init__(self, relation, terms, alias=None):
        self.relation = relation
        self.args = terms
        self.alias = alias
        self.term_type = TermType.COMPOSITE

    def __str__(self):
        return self.relation.name + '(' + ','.join([str(x) for x in self.args]) + ')'


class Atom(Term):
    def __init__(self, value):
        self.val = value
        self.term_type = TermType.ATOM

    def __str__(self):
        if type(self.val) is str:
            return '"' + self.val + '"'
        else:
            return str(self.val)


class Variable(Term):
    def __init__(self, name: str, var_type):
        self.var = name
        self.var_type = var_type
        self.term_type = TermType.VARIABLE

    def __str__(self):
        return '<' + self.var + '>'


'''
if __name__ == '__main__':
    node = Relation('node', ['id'], ['string'])
    edge = Relation('edge', ['src', 'dst'], ['node', 'node'])
    a1 = Atom('hello')
    a2 = Atom(123)
    a3 = Atom(1.23)
    v1 = Variable('X', node)
    v2 = Variable('Y', node)
    s1 = Variable('S', Relation.string())

    n1 = Composite(node, [a1])
    n2 = Composite(node, [s1])
    e1 = Composite(edge, [v1, v2])
    e2 = Composite(edge, [n1, v2])

    print(a1, a2, a3)
    print(v1, v2)
    print(n1, n2)
    print(e1, e2)
'''