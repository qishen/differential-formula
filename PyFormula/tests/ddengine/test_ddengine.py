import unittest
from pprint import pprint
import sys

from ddengine import Atom, Variable, Composite, DDExecuter
from executer.relation import *


class TermTestCase(unittest.TestCase):
    def setUp(self) -> None:
        self.node = BasicType('node', ['id'], ['string'])
        self.edge = BasicType('edge', ['src', 'dst'], ['node', 'node'])
        self.boolean = BuiltInType('Boolean')
        self.integer = BuiltInType('Integer')
        self.string = BuiltInType('String')
        self.v = Variable('hi.hello.nihao', self.node)

    def test_atom(self):
        a = Atom(1, self.integer)
        a1 = Atom(1, self.integer)
        a2 = Atom(2, self.integer)
        a3 = Atom('hello', self.string)
        a4 = Atom('hello', self.string)
        a5 = Atom(True, self.boolean)
        a6 = Atom(True, self.boolean)
        self.assertEqual(str(a), "1")
        self.assertEqual(str(a3), "\"hello\"")
        self.assertEqual(a3, a4)
        self.assertEqual(a5, a6)
        self.assertNotEqual(a3, a5)
        self.assertEqual(hash(a), hash(a1))
        self.assertNotEqual(hash(a1), hash(a2))

    def test_variable(self):
        v1 = Variable('hi.hello.nihao', self.node)
        v2 = Variable('hi', self.node)
        v3 = Variable('hello.world', self.node)
        [v4] = v1.get_variables()
        self.assertEqual(v1, v4)
        self.assertNotEqual(hash(v2), hash(v3))
        self.assertNotEqual(hash(self.v), hash(v2))
        self.assertEqual(hash(self.v), hash(v1))
        # self.assertEqual(self.v, v1)
        self.assertEqual(self.v.var, 'hi')
        self.assertListEqual(self.v.fragments, ['hello', 'nihao'])

    def test_composite(self):
        n = Composite(self.node, [self.v])
        n1 = Composite(self.node, [self.v])
        n2 = Composite(self.node, [Atom('hello', self.string)])
        e1 = Composite(self.edge, [n1, n2])
        e2 = Composite(self.edge, [n1, n1])
        e3 = Composite(self.edge, [n, n2])
        # print(e3)
        self.assertEqual(e1, e3)
        self.assertNotEqual(e1, e2)
        self.assertEqual(len(n), 1)
        self.assertEqual(len(e1), 2)
        self.assertNotEqual(hash(e1), hash(e2))
        self.assertNotEqual(hash(n1), hash(e1))
        self.assertEqual(hash(n), hash(n1))
        self.assertNotEqual(hash(n), hash(n2))

    def test_term_features(self):
        vn1 = Variable('a', self.node)
        vn2 = Variable('b', self.node)
        vs1 = Variable('s', self.string)
        n1 = Composite(self.node, [vs1])
        e1 = Composite(self.edge, [vn1, vn2])
        e2 = Composite(self.edge, [n1, vn1])
        [vn1x, vn2x] = e1.get_variables()
        [vs1y, vn1y] = e2.get_variables()
        self.assertEqual(vn1, vn1x)
        self.assertEqual(vn2, vn2x)
        self.assertEqual(vn1, vn1y)
        self.assertEqual(vs1, vs1y)

    def test_term_bindings(self):
        vn1 = Variable('a', self.node)
        ve1 = Composite(self.edge, [Variable('a', self.node), Composite(self.node, [Variable('b', self.string)])])
        g1 = Composite(self.node, [Atom('HELLO', self.string)])
        g2 = Composite(self.edge, [Composite(self.node, [Atom('nihao', self.string)]), Composite(self.node, [Atom('privet', self.string)])])
        print(vn1)
        print(ve1)
        print(g1)
        print(g2)
        bindings = vn1.get_bindings(g1)

        for key in bindings:
            print("%s : %s" % (key, bindings[key]))

        bindings2 = ve1.get_bindings(g2)
        for key in bindings2:
            print("%s : %s" % (key, bindings2[key]))

        pn1 = vn1.propagate_bindings(bindings)
        print(pn1)

        pn2 = ve1.propagate_bindings(bindings2)
        print(pn2)
