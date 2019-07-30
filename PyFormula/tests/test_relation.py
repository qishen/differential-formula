import unittest

from modules.relation import *


class RelationTestCase(unittest.TestCase):

    def test_relation(self):
        link = Relation('link', ['src', 'dst'], ["string", "string"])
        link.data['hello'] = 1
        link.delta_data['world'] = 2

        self.assertDictEqual(link.data, {'hello': 1})
        self.assertDictEqual(link.delta_data, {'world': 2})
        self.assertIn('hello', link.combined_data)
        self.assertIn('world', link.combined_data)

    def test_counter_chainmap(self):
        d1 = {'hello': 1, 'world': 2}
        d2 = {'hello': 3, 'world': -2, 'hi': 4}
        c = CounterChainMap(d1, d2)

        self.assertEqual(c['hello'], 4)
        self.assertEqual(c['hi'], 4)
        self.assertEqual(len(c), 2)
        self.assertNotIn('world', c)

    def test_set_diff_map(self):
        d3 = {'hello': 1, 'world': 3}
        d4 = {'world': 2, 'bug': 4}
        s = SetDiffMap(d3, d4)

        self.assertEqual(len(s), 1)
        self.assertIn('hello', s)
        self.assertNotIn('world', s)
        self.assertNotIn('bug', s)
