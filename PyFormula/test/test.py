import unittest
import logging

from modules.rule import Rule
from modules.relation import Relation
from modules.term import Atom, Variable, Composite
from modules.constraint import PredType, Predicate
from compiler import Compiler


class BaseLinkTestCase(unittest.TestCase):
    def setUp(self):
        # Define algebraic data type.
        self.link = Relation('link', ['src', 'dst'], ["string", "string"])
        self.hop = Relation('hop', ['src', 'dst'], ["string", "string"])
        self.tri_hop = Relation('tri_hop', ['src', 'dst'], ["string", "string"])
        self.only_tri_hop = Relation('only_tri_hop', ['src', 'dst'], ['string', 'string'])
        self.relations = [self.link, self.hop, self.tri_hop, self.only_tri_hop]

        '''
        Define terms that will be used in rules. 
        '''
        string_sort = Relation('string')
        self.link_x_z_term = Composite(self.link, [Variable('X', string_sort), Variable('Z', string_sort)])
        self.link_z_y_term = Composite(self.link, [Variable('Z', string_sort), Variable('Y', string_sort)])
        self.hop_x_y_term = Composite(self.hop, [Variable('X', string_sort), Variable('Y', string_sort)])
        self.hop_x_z_term = Composite(self.hop, [Variable('X', string_sort), Variable('Z', string_sort)])
        self.tri_hop_x_y_term = Composite(self.tri_hop, [Variable('X', string_sort), Variable('Y', string_sort)])
        self.only_tri_hop_x_y_term = Composite(self.only_tri_hop,
                                               [Variable('X', string_sort), Variable('Y', string_sort)])

        ''' 
        Turn terms into constraint predicates and and all of them are in original form.
        '''
        self.link_x_z = Predicate(self.link_x_z_term, PredType.ORIGINAL, False)
        self.link_z_y = Predicate(self.link_z_y_term, PredType.ORIGINAL, False)
        self.hop_x_y = Predicate(self.hop_x_y_term, PredType.ORIGINAL, False)
        self.negated_hop_x_y = Predicate(self.hop_x_y_term, PredType.ORIGINAL, True)
        self.hop_x_z = Predicate(self.hop_x_z_term, PredType.ORIGINAL, False)
        self.tri_hop_x_y = Predicate(self.tri_hop_x_y_term, PredType.ORIGINAL, False)
        self.only_tri_hop_x_y = Predicate(self.only_tri_hop_x_y_term, PredType.ORIGINAL, True)

        '''
        Rules composed by predicates
        (R1) hop(x,y) :- link(x,z), link(z,y).
        (R2) tri_hop(x,y) :- hop(x,z), link(z,y).
        (R3) only_tri_hop(x,y) :- tri_hop(x,y), no hop(x,y).
        '''
        self.hop_rule = Rule([self.hop_x_y], [self.link_x_z, self.link_z_y])
        self.tri_hop_rule = Rule([self.tri_hop_x_y], [self.hop_x_z, self.link_z_y])
        self.only_tri_hop_rule = Rule([self.only_tri_hop_x_y], [self.tri_hop_x_y, self.negated_hop_x_y])
        self.rules = [self.hop_rule, self.tri_hop_rule, self.only_tri_hop_rule]

        '''
        Initialize compiler with given relations and rules.
        '''
        self.compiler = Compiler(self.relations, self.rules)


class NonRecursiveLinkTestCase(BaseLinkTestCase):

    def setUp(self):
        super().setUp()

    #@unittest.skip("Skip temporarily")
    def test_first_input(self):
        link_facts_raw = [['a', 'b'], ['a', 'd'], ['d', 'c'], ['b', 'c'], ['c', 'h'], ['f', 'g']]
        link_facts = [Composite(self.link, [Atom(t[0]), Atom(t[1])]) for t in link_facts_raw]
        self.compiler.compile(link_facts)

        print('-------------------------------------')
        print('--- Print out initial model facts ---')
        print('-------------------------------------')
        self.compiler.print_all_facts()

        print('\n--- Test on incremental evaluation ---')
        c1 = Composite(self.link, [Atom('a'), Atom('b')])
        c2 = Composite(self.link, [Atom('d'), Atom('f')])
        c3 = Composite(self.link, [Atom('a'), Atom('f')])
        changes = {c1: -1, c2: 1, c3: 1}

        print('Make some changes to existing facts: ')
        for (term, count) in changes.items():
            if count > 0:
                print('Add ', term)
            else:
                print('Remove ', term)

        self.compiler.add_changes(changes)

        print('----------------------------------------------------------')
        print('--- Print out model facts after changes are propagated ---')
        print('----------------------------------------------------------')
        self.compiler.print_all_facts()

    @unittest.skip("Skip temporarily")
    def test_second_input(self):
        link_facts_raw = [['a', 'b'], ['a', 'e'], ['a', 'f'], ['a', 'g'], ['b', 'c'], ['c', 'd'], ['c', 'k'],
                          ['e', 'd'], ['f', 'd'], ['g', 'h'], ['h', 'k']]
        link_facts = [Composite(self.link, [Atom(t[0]), Atom(t[1])]) for t in link_facts_raw]
        self.compiler.compile(link_facts)

        print('-------------------------------------')
        print('--- Print out initial model facts ---')
        print('-------------------------------------')
        self.compiler.print_all_facts()

        print('\n--- Test on incremental evaluation ---')
        c1 = Composite(self.link, [Atom('b'), Atom('k')])
        changes = {c1: 1}

        print('Make some changes to existing facts: ')
        for (term, count) in changes.items():
            if count > 0:
                print('Add ', term)
            else:
                print('Remove ', term)

        self.compiler.add_changes(changes)

        print('----------------------------------------------------------')
        print('--- Print out model facts after changes are propagated ---')
        print('----------------------------------------------------------')
        self.compiler.print_all_facts()


class RecursiveLinkClass(BaseLinkTestCase):
    def setUp(self):
        super().setUp()

    def test_me(self):
        pass

