using System;
using System.Collections.Generic;
using System.Text;

using Antlr4.Runtime;

namespace Microsoft.Formula.Core.Parser.Nodes
{
    public class Constraint : Node
    {
        public bool IsNegated { get; }

        public RelKind Op
        {
            get;
            private set;
        }

        public Node Arg1
        {
            get;
            private set;
        }

        public Node Arg2
        {
            get;
            private set;
        }

        public Constraint(ParserRuleContext sourceLocation, RelKind op, Node arg1, Node arg2) : base(sourceLocation)
        {
            Arg1 = arg1;
            Arg2 = arg2;
            Op = op;
            IsNegated = false;
        }

        public Constraint(ParserRuleContext sourceLocation, bool negated, Node arg) : base(sourceLocation)
        {
            IsNegated = negated;
            Arg1 = arg;
        }

        public override NodeKind NodeKind
        {
            get { return NodeKind.Constraint; }
        }
    }
}
