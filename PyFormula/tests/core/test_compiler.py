import unittest

from compiler import Compiler
from executer.constraint import PredType, Predicate
from executer.relation import BasicType
from executer.rule import Rule
from executer.term import Atom, Variable, Composite


class RuleTransformationTestCase(unittest.TestCase):
    def setUp(self) -> None:
        self.compiler = Compiler()

    @unittest.skip("Skip temporarily")
    def test_rules_stratification(self):
        self.domain_str = '''
            domain ReachableGraph
            {
                link ::= new (src: String, dst: String).
                node ::= new (x: String).
                reachable ::= new (src: String, dst: String).
                unreachable ::= new (src: String, dst: String).
                 
                reachable(X,Y) :- link(X,Y)
                reachable(X,Y) :- link(X,Z), reachable(Z,Y)
                node(X) :- link(X,Y)
                node(Y) :- link(X,Y)
                unreachable(X,Y) :- node(X), node(Y), not reachable(X,Y)
            }
        '''

        model_str = '''
            model g of ReachableGraph
            {
                
            }
        '''

        file_str = self.domain_str + model_str
        self.compiler.parse(file_str=file_str)
        self.compiler.execute_model('g')


class BaseLinkTestCase(unittest.TestCase):
    def setUp(self):
        self.domain_str = '''
            domain LinkGraph
            {
                link ::= new (src: String, dst: String).
                hop ::= new (src: String, dst: String).
                tri_hop ::= new (src: String, dst: String).
                only_tri_hop ::= new (src: String, dst: String).
                
                hop(x,y) :- link(x,z), link(z,y).
                tri_hop(x,y) :- hop(x,z), link(z,y).
                only_tri_hop(x,y) :- tri_hop(x,y), no hop(x,y).
            }
        '''


class NonRecursiveLinkTestCase(BaseLinkTestCase):
    def setUp(self):
        super().setUp()
        self.compiler = Compiler()
        self.logger = self.compiler.logger

    @unittest.skip("Skip temporarily")
    def test_first_input(self):
        model_str = '''
        model g1 of LinkGraph
        {
            l1 is link("a", "b").
            l2 is link("a", "d").
            l3 is link("d", "c").
            l4 is link("b", "c").
            l5 is link("c", "h").
            l6 is link("f", "g").
        }'''

        file_str = self.domain_str + model_str
        self.compiler.parse(file_str=file_str)
        self.compiler.execute_model('g1')

        self.logger.info('-------------------------------------')
        self.logger.info('--- Print out initial model facts ---')
        self.logger.info('-------------------------------------')
        self.compiler.print_all_facts('g1')

        self.logger.info('\n--- Test on incremental evaluation ---')
        domain = self.compiler.find_domain_by_name('LinkGraph')
        link = domain.type_map['link']
        c1 = Composite(link, [Atom('a'), Atom('b')])
        c2 = Composite(link, [Atom('d'), Atom('f')])
        c3 = Composite(link, [Atom('a'), Atom('f')])
        changes = {c1: -1, c2: 1, c3: 1}

        self.logger.info('Make some changes to existing facts: ')
        for (term, count) in changes.items():
            if count > 0:
                self.logger.info('Add ' + str(term))
            else:
                self.logger.info('Remove ' + str(term))
        self.logger.info('\n')

        self.compiler.make_changes_and_execute('g1', changes)

        self.logger.info('----------------------------------------------------------')
        self.logger.info('--- Print out model facts after changes are propagated ---')
        self.logger.info('----------------------------------------------------------')
        self.compiler.print_all_facts('g1')

    #@unittest.skip("Skip temporarily")
    def test_second_input(self):
        model_str = '''
            model g2 of LinkGraph
            {
                link("a", "b").
                link("a", "e").
                link("a", "f").
                link("a", "g").
                link("b", "c").
                link("c", "d").
                link("c", "k").
                link("e", "d").
                link("f", "d").
                link("g", "h").
                link("h", "k").
            }
        '''

        file_str = self.domain_str + model_str
        self.compiler.parse(file_str=file_str)
        self.compiler.execute_model('g2')

        self.logger.info('-------------------------------------')
        self.logger.info('--- Print out initial model facts ---')
        self.logger.info('-------------------------------------')
        self.compiler.print_all_facts('g2')

        self.logger.info('\n--- Test on incremental evaluation ---')
        domain = self.compiler.find_domain_by_name('LinkGraph')
        link = domain.type_map['link']
        c1 = Composite(link, [Atom('b'), Atom('k')])
        changes = {c1: 1}

        self.logger.info('Make some changes to existing facts: ')
        for (term, count) in changes.items():
            if count > 0:
                self.logger.info('Add ' + str(term))
            else:
                self.logger.info('Remove ' + str(term))
        self.logger.info('\n')

        self.compiler.make_changes_and_execute('g2', changes)

        self.logger.info('----------------------------------------------------------')
        self.logger.info('--- Print out model facts after changes are propagated ---')
        self.logger.info('----------------------------------------------------------')
        self.compiler.print_all_facts('g2')


class FullyConnectedGraphTestCase(unittest.TestCase):
    def setUp(self):
        self.domain_str = '''
            domain PGraph
            {
                link ::= new (src: String, dst: String).
                p ::= new (x: String, y: String, z: String).
                p(x,y,z) :- link(x,z), link(z,y), link(y,z).  
            }
        '''
        # Fully connected graph has too many edges and would be easy to enter data using APIs instead of writing models.
        self.compiler = Compiler()
        self.logger = self.compiler.logger

    @unittest.skip("Skip temporarily")
    def test_fully_connected_graph(self):
        self.compiler.parse(file_str=self.domain_str)

        domain = self.compiler.find_domain_by_name('PGraph')
        link = domain.type_map['link']
        nodes_raw = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j']
        changes = {}
        for i in range(10):
            for j in range(10):
                composite = Composite(link, [Atom(nodes_raw[i]), Atom(nodes_raw[j])])
                changes[composite] = 1

        model = self.compiler.generate_model('PGraph', 'pg1')
        model.add_changes(changes)

        self.logger.info('-------------------------------------')
        self.logger.info('--- Print out initial model facts ---')
        self.logger.info('-------------------------------------')
        self.compiler.print_all_facts('pg1')
        p = domain.type_map['p']
        p_index = model.type_index_map[p]
        print('Number of facts in p relation is %s' % p_index.facts_count())
        self.assertEqual(p_index.facts_count(), 1000)


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




