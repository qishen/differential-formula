﻿using System;
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
                    Node ::= new (id: Integer).
                    Edge ::= new (src: Node, dst: Node).
                }
            ";

            ICharStream stream = new AntlrInputStream(input);
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
