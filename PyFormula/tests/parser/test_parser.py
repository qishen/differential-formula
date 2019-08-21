import unittest
from pathlib import Path, PurePath

from compiler import Compiler


class ParserTestCase(unittest.TestCase):
    def setUp(self):
        self.compiler = Compiler()

    def tearDown(self) -> None:
        self.compiler.clear_all()

    @unittest.skip('Temporarily skip')
    def test_parse_str(self):
        formula_str = '''
            domain Graph
            {
                Node ::= new (id: String).
                Edge ::= new (src: Node, dst: Node).
                Thing ::= Node + Edge + {NIL, NONE}.
                Edge(Node(a), Node("hello")) :- Edge(Node(a), c), Edge(c, Node("hello")), 
                    count({n | n is Edge(c, b), c is Node("hello")}) = 1.
                
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

    def test_parse_file(self):
        current_dir = Path(__file__).parent
        samples_dir = current_dir.parent.parent
        formula_file = samples_dir.joinpath('samples/graphs.4ml')
        self.compiler.parse(filename=formula_file)

