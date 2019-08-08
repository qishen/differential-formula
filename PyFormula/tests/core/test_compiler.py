import unittest

from compiler import Compiler
from executer.constraint import PredType, Predicate
from executer.relation import BasicType
from executer.rule import Rule
from executer.term import Atom, Variable, Composite


class RuleTransformationTestCase(unittest.TestCase):
    def setUp(self) -> None:
        pass

    @unittest.skip("Skip temporarily")
    def test_rules_stratification(self):
        string_sort = BasicType('string')
        link = BasicType('link', ['src', 'dst'], ["string", "string"])
        reachable = BasicType('reachable', ['src', 'dst'], ['string', 'string'])
        node = BasicType('node', ['x'], ['string'])
        unreachable = BasicType('unreachable', ['src', 'dst'], ['string', 'string'])

        relations = [link, reachable, node, unreachable]

        '''
        reachable(X,Y) :- link(X,Y)
        reachable(X,Y) :- link(X,Z), reachable(Z,Y)
        node(X) :- link(X,Y)
        node(Y) :- link(X,Y)
        unreachable(X,Y) :- node(X), node(Y), not reachable(X,Y)
        '''
        link_x_y_term = Composite(link, [Variable('X', string_sort), Variable('Y', string_sort)])
        reachable_x_y_term = Composite(reachable, [Variable('X', string_sort), Variable('Y', string_sort)])

        link_x_z_term = Composite(link, [Variable('X', string_sort), Variable('Z', string_sort)])
        reachable_z_y_term = Composite(reachable, [Variable('Z', string_sort), Variable('Y', string_sort)])

        node_x_term = Composite(node, [Variable('X', string_sort)])
        node_y_term = Composite(node, [Variable('Y', string_sort)])
        unreachable_x_y_term = Composite(unreachable, [Variable('X', string_sort), Variable('Y', string_sort)])

        rule1 = Rule([Predicate(reachable_x_y_term)], [Predicate(link_x_y_term)])
        rule2 = Rule([Predicate(reachable_x_y_term)], [Predicate(link_x_z_term), Predicate(reachable_z_y_term)])
        rule3 = Rule([Predicate(node_x_term)], [Predicate(link_x_y_term)])
        rule4 = Rule([Predicate(node_y_term)], [Predicate(link_x_y_term)])
        rule5 = Rule([Predicate(unreachable_x_y_term)], [Predicate(node_x_term), Predicate(node_y_term),
                                                         Predicate(reachable_x_y_term, negated=True)])
        rules = [rule1, rule2, rule3, rule4, rule5]

        compiler = Compiler(relations, rules)
        compiler.stratify_rules()


class BaseLinkTestCase(unittest.TestCase):
    def setUp(self):
        # Define algebraic data type.
        self.link = BasicType('link', ['src', 'dst'], ["string", "string"])
        self.hop = BasicType('hop', ['src', 'dst'], ["string", "string"])
        self.tri_hop = BasicType('tri_hop', ['src', 'dst'], ["string", "string"])
        self.only_tri_hop = BasicType('only_tri_hop', ['src', 'dst'], ['string', 'string'])

        '''
        Define terms that will be used in rules. 
        '''
        string_sort = BasicType('string')
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


class FullyConnectedGraphTestCase(BaseLinkTestCase):

    def setUp(self):
        super().setUp()
        string_sort = BasicType('string')
        self.p = BasicType('p', ['x', 'y', 'z'], ['string', 'string', 'string'])
        self.link_y_z_term = Composite(self.link, [Variable('Y', string_sort), Variable('Z', string_sort)])
        self.p_x_y_z_term = Composite(self.p, [Variable('X', string_sort), Variable('Y', string_sort), Variable('Z', string_sort)])
        self.link_y_z = Predicate(self.link_y_z_term)
        self.p_x_y_z = Predicate(self.p_x_y_z_term)
        ''' p(x,y,z) :- link(x,z), link(z,y), link(y,z). '''
        p_rule = Rule([self.p_x_y_z], [self.link_x_z, self.link_z_y, self.link_y_z])
        rules = [p_rule]
        relations = [self.link, self.p]
        self.compiler = Compiler(relations, rules)
        self.logger = self.compiler.logger

    #@unittest.skip("Skip temporarily")
    def test_fully_connected_graph(self):
        nodes_raw = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j']
        link_facts = []
        for i in range(10):
            for j in range(10):
                composite = Composite(self.link, [Atom(nodes_raw[i]), Atom(nodes_raw[j])])
                link_facts.append(composite)
        self.compiler.compile(link_facts)

        self.logger.info('-------------------------------------')
        self.logger.info('--- Print out initial model facts ---')
        self.logger.info('-------------------------------------')
        #self.compiler.print_all_facts()

        print('Number of facts in p relation is %s' % self.p.facts_count())

    @unittest.skip("Skip temporarily")
    def test_small_graph(self):
        link_facts_raw = [['a', 'b'], ['a', 'c'], ['a', 'd'], ['a', 'e'],
                          ['b', 'f'], ['c', 'g'], ['d', 'h'], ['e', 'i']]
        link_facts = [Composite(self.link, [Atom(t[0]), Atom(t[1])]) for t in link_facts_raw]
        self.compiler.compile(link_facts)

        self.logger.info('-------------------------------------')
        self.logger.info('--- Print out initial model facts ---')
        self.logger.info('-------------------------------------')
        self.compiler.print_all_facts()


class NonRecursiveLinkTestCase(BaseLinkTestCase):

    def setUp(self):
        super().setUp()
        rules = [self.hop_rule, self.tri_hop_rule, self.only_tri_hop_rule]
        relations = [self.link, self.hop, self.tri_hop, self.only_tri_hop]
        self.compiler = Compiler(relations, rules)
        self.logger = self.compiler.logger

    #@unittest.skip("Skip temporarily")
    def test_first_input(self):
        link_facts_raw = [['a', 'b'], ['a', 'd'], ['d', 'c'], ['b', 'c'], ['c', 'h'], ['f', 'g']]
        link_facts = [Composite(self.link, [Atom(t[0]), Atom(t[1])]) for t in link_facts_raw]
        self.compiler.compile(link_facts)

        self.logger.info('-------------------------------------')
        self.logger.info('--- Print out initial model facts ---')
        self.logger.info('-------------------------------------')
        self.compiler.print_all_facts()

        self.logger.info('\n--- Test on incremental evaluation ---')
        c1 = Composite(self.link, [Atom('a'), Atom('b')])
        c2 = Composite(self.link, [Atom('d'), Atom('f')])
        c3 = Composite(self.link, [Atom('a'), Atom('f')])
        changes = {c1: -1, c2: 1, c3: 1}

        self.logger.info('Make some changes to existing facts: ')
        for (term, count) in changes.items():
            if count > 0:
                self.logger.info('Add ' + str(term))
            else:
                self.logger.info('Remove ' + str(term))
        self.logger.info('\n')

        self.compiler.add_changes(changes)

        self.logger.info('----------------------------------------------------------')
        self.logger.info('--- Print out model facts after changes are propagated ---')
        self.logger.info('----------------------------------------------------------')
        self.compiler.print_all_facts()

    @unittest.skip("Skip temporarily")
    def test_second_input(self):

        link_facts_raw = [['a', 'b'], ['a', 'e'], ['a', 'f'], ['a', 'g'], ['b', 'c'], ['c', 'd'], ['c', 'k'],
                          ['e', 'd'], ['f', 'd'], ['g', 'h'], ['h', 'k']]
        link_facts = [Composite(self.link, [Atom(t[0]), Atom(t[1])]) for t in link_facts_raw]
        self.compiler.compile(link_facts)

        self.logger.info('-------------------------------------')
        self.logger.info('--- Print out initial model facts ---')
        self.logger.info('-------------------------------------')
        self.compiler.print_all_facts()

        self.logger.info('\n--- Test on incremental evaluation ---')
        c1 = Composite(self.link, [Atom('b'), Atom('k')])
        changes = {c1: 1}

        self.logger.info('Make some changes to existing facts: ')
        for (term, count) in changes.items():
            if count > 0:
                self.logger.info('Add ' + str(term))
            else:
                self.logger.info('Remove ' + str(term))
        self.logger.info('\n')

        self.compiler.add_changes(changes)

        self.logger.info('----------------------------------------------------------')
        self.logger.info('--- Print out model facts after changes are propagated ---')
        self.logger.info('----------------------------------------------------------')
        self.compiler.print_all_facts()


class RecursiveLinkClass(BaseLinkTestCase):
    def setUp(self):
        super().setUp()
        string_sort = BasicType('string')
        self.link_x_y_term = Composite(self.link, [Variable('X', string_sort), Variable('Y', string_sort)])
        self.link_x_y = Predicate(self.link_x_y_term)
        self.recursive_link_rule = Rule([self.link_x_y], [self.link_x_z, self.link_z_y])
        rules = [self.recursive_link_rule]
        relations = [self.link]
        self.compiler = Compiler(relations, rules)
        self.logger = self.compiler.logger

    @unittest.skip("Skip temporarily")
    def test_small_tc_graph(self):
        link_facts_raw = [['a', 'b'], ['b', 'c'], ['c', 'd'], ['d', 'e'], ['e', 'f']]
        link_facts = [Composite(self.link, [Atom(t[0]), Atom(t[1])]) for t in link_facts_raw]
        self.compiler.compile(link_facts)

        self.logger.info('----------------------------------------------------------')
        self.logger.info('--- Print out model facts after changes are propagated ---')
        self.logger.info('----------------------------------------------------------')
        self.compiler.print_all_facts()

    @unittest.skip("Skip temporarily")
    def test_graph_with_dred(self):
        link_facts_raw = [['a', 'b'], ['b', 'c'], ['c', 'd'], ['c', 'c']]
        link_facts = [Composite(self.link, [Atom(t[0]), Atom(t[1])]) for t in link_facts_raw]
        self.compiler.compile(link_facts)

        self.logger.info('----------------------------------------------------------')
        self.logger.info('--- Print out model facts after changes are propagated ---')
        self.logger.info('----------------------------------------------------------')
        self.compiler.print_all_facts()




