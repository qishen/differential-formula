using System;
using System.Collections.Generic;
using System.Text;

using Antlr4.Runtime;

namespace Microsoft.Formula.Core.Parser.Nodes
{
    public class WrappedExpr : Node
    {
        public WrappedExpr(ParserRuleContext sourceLocation) : base(sourceLocation)
        {

        }

        public override NodeKind NodeKind
        {
            get { return NodeKind.WrappedExpr; }
        }
    }
}
