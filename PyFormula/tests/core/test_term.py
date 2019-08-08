import unittest

from executer.term import *
from executer.relation import *


class TermTestCase(unittest.TestCase):
    def setUp(self):
        self.node = BasicType('node', ['id'], ['string'])

    def test_variables(self):
        v1 = Variable('hello.my.friend', None)
        self.assertEqual(v1.var, 'hello')
        self.assertEqual(v1.fragments, ['my', 'friend'])

    def test_atoms(self):
        self.assertEqual(Atom('hello'), Atom('hello'))
        self.assertNotEqual(Atom('hello'), Atom('world'))

    def test_composite(self):
        hello = Composite(self.node, [Atom('hello')])
        world = Composite(self.node, [Atom('world')])
        goodbye = Composite(self.node, [Atom('goodbye')])
        world_clone = Composite(self.node, [Atom('world')])

        self.assertNotEqual(hello, goodbye)
        self.assertEqual(world, world_clone)

    def test_bindings_propagation(self):
        link = BasicType('link', ['src', 'dst'], ["string", "string"])
        string_sort = BaseType('string')
        link_x_z_term = Composite(link, [Variable('X', string_sort), Variable('Z', string_sort)])
        bindings = {Variable('X', string_sort): Atom('hello'), Variable('Z', string_sort): Atom('world')}
        t = link_x_z_term.propagate_bindings(bindings)
        self.assertEqual(t, Composite(link, [Atom('hello'), Atom('world')]))
