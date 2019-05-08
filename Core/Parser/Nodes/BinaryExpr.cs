using System;
using System.Collections.Generic;
using System.Text;

using Antlr4.Runtime;

namespace Microsoft.Formula.Core.Parser.Nodes
{
    public class BinaryExpr : Node
    {
        public BinaryExpr(ParserRuleContext sourceLocation) : base(sourceLocation)
        {

        }

        public override NodeKind NodeKind
        {
            get { return NodeKind.BinaryExpr; }
        }
    }
}