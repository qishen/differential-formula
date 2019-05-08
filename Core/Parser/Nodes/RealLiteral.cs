using System;
using System.Collections.Generic;
using System.Text;

using Antlr4.Runtime;

namespace Microsoft.Formula.Core.Parser.Nodes
{
    class RealLiteral : Node
    {
        public RealLiteral(ParserRuleContext sourceLocation, string realString) : base(sourceLocation)
        {

        }

        public override NodeKind NodeKind
        {
            get { return NodeKind.RealLiteral; }
        }
    }
}
