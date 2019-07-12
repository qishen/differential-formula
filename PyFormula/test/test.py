import unittest

from modules.rule import Rule
from modules.relation import Relation
from modules.term import Atom, Variable, Composite
from modules.constraint import PredType, Predicate
from compiler import Compiler


class LinkTransitiveClosureTestCase(unittest.TestCase):

    def setUp(self):
        self.link = Relation('self.link', ['src', 'dst'], ["string", "string"])
        self.hop = Relation('hop', ['src', 'dst'], ["string", "string"])
        self.tri_hop = Relation('tri_hop', ['src', 'dst'], ["string", "string"])
        self.only_tri_hop = Relation('self.only_tri_hop', ['src', 'dst'], ['string', 'string'])
        self.relations = [self.link, self.hop, self.tri_hop, self.only_tri_hop]

        '''
        Define predicates used in rules and all of them are in original form
        '''
        string_sort = Relation('string')
        self.link_x_z_term = Composite(self.link, [Variable('X', string_sort), Variable('Z', string_sort)])
        self.link_z_y_term = Composite(self.link, [Variable('Z', string_sort), Variable('Y', string_sort)])
        self.hop_x_y_term = Composite(self.hop, [Variable('X', string_sort), Variable('Y', string_sort)])
        self.hop_x_z_term = Composite(self.hop, [Variable('X', string_sort), Variable('Z', string_sort)])
        self.tri_hop_x_y_term = Composite(self.tri_hop, [Variable('X', string_sort), Variable('Y', string_sort)])
        self.only_tri_hop_x_y_term = Composite(self.only_tri_hop, [Variable('Z', string_sort), Variable('Y', string_sort)])
        
        ''' 
        Turn terms into constraint predicates
        '''
        self.link_x_y = Predicate(self.link_x_z_term, PredType.ORIGINAL, False)
        self.link_z_y = Predicate(self.link_z_y_term, PredType.ORIGINAL, False)
        self.hop_x_y = Predicate(self.hop_x_y_term, PredType.ORIGINAL, False)
        self.negated_hop_x_y = Predicate(self.hop_x_y_term, PredType.ORIGINAL, True)
        self.hop_x_z = Predicate(self.hop_x_z_term, PredType.ORIGINAL, False)
        self.tri_hop_x_y = Predicate(self.tri_hop_x_y_term, PredType.ORIGINAL, False)
        self.only_tri_hop_x_y = Predicate(self.only_tri_hop_x_y_term, PredType.ORIGINAL, True)

        '''
        Rules composed by predicates
        '''
        self.hop_rule = Rule([self.hop_x_y], [self.link_x_y, self.link_z_y])
        self.tri_hop_rule = Rule([self.tri_hop_x_y], [self.hop_x_z, self.link_z_y])
        self.only_tri_hop_rule = Rule([self.only_tri_hop_x_y], [self.tri_hop_x_y, self.negated_hop_x_y])
        self.rules = [self.hop_rule, self.tri_hop_rule, self.only_tri_hop_rule]

    def test_something(self):
        pass




