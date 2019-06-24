using System;
using System.Collections.Generic;
using System.Text;

using Antlr4.Runtime;

namespace Microsoft.Formula.Core.Parser.Nodes
{
    public class Conjunction : Node
    {
        public List<Node> Constraints { get; }

        public Conjunction(ParserRuleContext sourceLocation, List<Node> constraints)
        {
            Constraints = constraints;
        }

        public override NodeKind NodeKind
        {
            get { return NodeKind.Conjunction; }
        }
    }
}
