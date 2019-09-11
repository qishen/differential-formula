import unittest
from pathlib import Path, PurePath

from compiler import Compiler


class ParserTestCase(unittest.TestCase):
    def setUp(self):
        self.compiler = Compiler()

    def tearDown(self) -> None:
        self.compiler.clear_all()

    def test_parse_simple_graph_in_string(self):
        formula_str = '''
            domain Graph
            {
                // Add some comments here ^_^
                Node ::= new (id: String).
                Nodes ::= new (item: Node, nxt: any Nodes + {NIL}).
                Edge ::= new (src: Node, dst: Node).
                Thing ::= Node + Edge + {NIL, NONE, 1..100}.
                
                //Edge(Node(a), Node("hello")) :- Edge(Node(a), c), Edge(c, Node("hello")), 
                    //count({n | n is Edge(c, b), c is Node("hello")}) = 1.
                    
                hasNode :- Node(a).
                hasThing :- Edge(a, b), hasNode.
                //hasOnlyEdge :- Edge(a, b), no hasNode.
                
                conforms no Edge(a, a).
            }
            
            model g of Graph
            {
                n1 is Node("hello").
                e1 is Edge(n1, Node("hi")).
                e2 is Edge(Node("world"), n1).
            }
        '''
        self.compiler.parse(file_str=formula_str)
        self.compiler.find_domain_by_name('Graph')

    def test_parse_graphs_in_file(self):
        current_dir = Path(__file__).parent
        samples_dir = current_dir.parent.parent
        formula_file = samples_dir.joinpath('samples/graphs.4ml')
        self.compiler.parse(filename=formula_file)

    def test_parse_weighted_graph_in_file(self):
        current_dir = Path(__file__).parent
        samples_dir = current_dir.parent.parent
        formula_file = samples_dir.joinpath('samples/weighted_graph.4ml')
        self.compiler.parse(filename=formula_file)

