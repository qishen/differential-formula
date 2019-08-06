import unittest

from grammar.visitor import ExprVisitor
from compiler import Compiler


class ParserTestCase(unittest.TestCase):
    def setUp(self):
        self.compiler = Compiler([],[])

    def test_visitor(self):
        formula_str = '''
        domain Graph
        {
            Node ::= new (id: String).
            Edge ::= new (src: Node, dst: Node).
            Edge(a, b) :- Edge(a, c), Edge(c, b).
        }
        
        model g of Graph
        {
            n1 is Node("hello").
        }
        '''
        self.compiler.parse_string(formula_str)

