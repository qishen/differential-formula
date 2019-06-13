using System;
using System.Collections.Generic;
using System.Text;

using Antlr4.Runtime;

namespace Microsoft.Formula.Core.Parser.Nodes
{
    public class RelConstr : Node
    {
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

        public RelConstr(RelKind op, Node arg1, Node arg2)
        {
            Op = op;
            Arg1 = arg1;
            Arg2 = arg2;
        }

        public override NodeKind NodeKind
        {
            get { return NodeKind.RelConstr; }
        }
    }
}
