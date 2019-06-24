using System;
using System.Collections.Generic;
using System.Text;

using Antlr4.Runtime;

namespace Microsoft.Formula.Core.Parser.Nodes
{
    public class Rule : Node
    {
        public List<Node> Head { get; }
        public Disjunction Body { get; }
        public Rule(ParserRuleContext sourceLocation, List<Node> head, Disjunction body) : base(sourceLocation)
        {
            Head = head;
            Body = body;
        }

        public override NodeKind NodeKind
        {
            get { return NodeKind.Rule; }
        }
    }
}
