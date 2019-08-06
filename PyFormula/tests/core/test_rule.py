import unittest

from executer.constraint import Predicate
from executer.relation import BasicType
from executer.rule import Rule, Bindings, BindingsCounter
from executer.term import Atom, Variable, Composite


class BindingsTestCase(unittest.TestCase):
    def setUp(self):
        self.node = BasicType('node', ['id'], ['string'])
        self.edge = BasicType('edge', ['src', 'dst'], ['node', 'node'])
        self.x = Variable('X', self.node)
        self.y = Variable('Y', self.node)
        self.z = Variable('Z', self.node)
        self.hello = Composite(self.node, [Atom('hello')])
        self.world = Composite(self.node, [Atom('world')])
        self.goodbye = Composite(self.node, [Atom('goodbye')])
        self.world_clone = Composite(self.node, [Atom('world')])

    def test_bindings_equality(self):
        d1 = Bindings({self.x: self.hello, self.y: self.world})
        d1_clone = Bindings({self.x: self.hello, self.y: self.world})
        d2 = Bindings({self.x: self.hello, self.z: self.world})
        d3 = Bindings({self.x: self.hello, self.y: self.world_clone})
        d3_clone = Bindings({self.x: self.hello, self.y: self.world_clone})
        d4 = Bindings({self.x: self.hello, self.y: self.world, self.z: self.goodbye})
        d5 = Bindings({self.x: self.hello, self.y: self.world, self.z: self.goodbye})

        self.assertEqual(d1, d3)
        self.assertEqual(d1, d1_clone)
        self.assertNotEqual(d1, d2)
        self.assertEqual(d3, d3_clone)
        self.assertNotEqual(d1, d4)
        self.assertEqual(d4, d5)
        d5[self.z] = self.hello
        self.assertNotEqual(d4, d5)

    def test_bindings_counter(self):
        counter = BindingsCounter()
        d1 = Bindings({self.x: self.hello, self.y: self.world, self.z: self.goodbye})
        d2 = Bindings({self.x: self.hello, self.z: self.world})
        d3 = Bindings({self.x: self.world, self.y: self.world})
        counter.update({d1: 1, d2: 2, d3: 3})

        self.assertEqual(counter[d1], 1)
        counter[d1] = -100
        self.assertEqual(counter[d1], -100)


class RuleTestCase(unittest.TestCase):
    def setUp(self):
        node = BasicType('node', ['id'], ['string'])
        edge = BasicType('edge', ['src', 'dst'], ['node', 'node'])
        hop = BasicType('hop', ['src', 'dst'], ['node', 'node'])
        six_hop = BasicType('six_hop', ['src', 'dst'], ['node', 'node'])

        relations = [node, edge, hop, six_hop]

        edge_x_y_term = Composite(edge, [Variable('X', node), Variable('Y', node)])
        edge_y_z_term = Composite(edge, [Variable('Y', node), Variable('Z', node)])
        hop_x_z_term = Composite(hop, [Variable('X', node), Variable('Z', node)])

        six_hop_e_k_term = Composite(six_hop, [Variable('E', node), Variable('K', node)])
        edge_e_f_term = Composite(edge, [Variable('E', node), Variable('F', node)])
        edge_f_g_term = Composite(edge, [Variable('F', node), Variable('G', node)])
        edge_g_h_term = Composite(edge, [Variable('G', node), Variable('H', node)])
        edge_h_i_term = Composite(edge, [Variable('H', node), Variable('I', node)])
        edge_i_j_term = Composite(edge, [Variable('I', node), Variable('J', node)])
        edge_j_k_term = Composite(edge, [Variable('J', node), Variable('K', node)])

        edge_x_y = Predicate(edge_x_y_term)
        edge_y_z = Predicate(edge_y_z_term)
        hop_x_z = Predicate(hop_x_z_term)

        six_hop_e_k = Predicate(six_hop_e_k_term)
        edge_e_f = Predicate(edge_e_f_term)
        edge_f_g = Predicate(edge_f_g_term)
        edge_g_h = Predicate(edge_g_h_term)
        edge_h_i = Predicate(edge_h_i_term)
        edge_i_j = Predicate(edge_i_j_term)
        edge_j_k = Predicate(edge_j_k_term)

        self.hop_rule = Rule([hop_x_z], [edge_x_y, edge_y_z])
        self.six_hop_rule = Rule([six_hop_e_k], [edge_e_f, edge_f_g, edge_g_h, edge_h_i, edge_i_j, edge_j_k])
        self.six_hop_rule_bad = Rule([six_hop_e_k], [edge_f_g, edge_e_f, edge_h_i, edge_g_h, edge_j_k, edge_i_j])

    def test_constraints_order(self):
        self.assertEqual(len(self.hop_rule.body), 2)
        self.assertEqual(len(self.six_hop_rule_bad.body), 6)
        self.assertEqual(len(self.six_hop_rule.body), 6)
