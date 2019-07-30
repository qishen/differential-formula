import unittest

from modules.term import *


class TermTestCase(unittest.TestCase):
    def setUp(self):
        self.node = Relation('node', ['id'], ['string'])

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
        link = Relation('link', ['src', 'dst'], ["string", "string"])
        string_sort = Relation('string')
        link_x_z_term = Composite(link, [Variable('X', string_sort), Variable('Z', string_sort)])
        bindings = {Variable('X', string_sort): Atom('hello'), Variable('Z', string_sort): Atom('world')}
        t = link_x_z_term.propagate_bindings(bindings)
        self.assertEqual(t, Composite(link, [Atom('hello'), Atom('world')]))
