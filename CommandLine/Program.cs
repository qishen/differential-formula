using System;
using Microsoft.Formula.Core.Parser;
using Microsoft.Formula.Core.Parser.Nodes;
using Microsoft.Formula.Core.Parser.Grammar;

using Antlr4.Runtime;
using Antlr4.Runtime.Tree;

namespace CommandLine
{
    class Program
    {
        static void Main(string[] args)
        {
            string input = @"
                domain Graph
                {
                    Node ::= new(id: Integer).
                    Edge ::= new(src: Node, dst: Node).
                }
            ";

            string input2 = @"
domain Graph
{
    Node ::= new (id: Integer).
    Edge ::= new (src: Node, dst: Node).
    Thing ::= Node + Edge + {1, 2, 3}.
    
    path ::= (src: Node, dst: Node).
    path(a, b) :- Edge(a, b).
    path(a, c) :- path(a, b), path(b, c).
}

model g of Graph
{
    v1 is Node(1).
    Node(2).
    e1 is Edge(Node(1), Node(2)).
    e2 is Edge(v1, v2).
}
            ";

            ICharStream stream = new AntlrInputStream(input2);
            FormulaLexer lexer = new FormulaLexer(stream);
            ITokenStream tokens = new CommonTokenStream(lexer);

            FormulaParser parser = new FormulaParser(tokens);
            parser.BuildParseTree = true;
            ParserRuleContext context = parser.program();
            IParseTree tree = context.children[0];
            

            ExprVisitor visitor = new ExprVisitor();
            object node = visitor.Visit(context);
        }
    }
}
