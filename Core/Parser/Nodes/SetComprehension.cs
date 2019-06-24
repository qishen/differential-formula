using System;
using System.Collections.Generic;
using System.Text;

using Antlr4.Runtime;

namespace Microsoft.Formula.Core.Parser.Nodes
{
    public class SetComprehension : Node
    {
        public LinkedList<Node> Head { get; }
        public LinkedList<Node> Body { get; }

        public SetComprehension(ParserRuleContext sourceLocation) : base(sourceLocation)
        {

        }

        public override NodeKind NodeKind
        {
            get { return NodeKind.SetComprehension; }
        }
    }
}
