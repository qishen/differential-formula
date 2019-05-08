using System;
using System.Collections.Generic;
using System.Text;

using Antlr4.Runtime;

namespace Microsoft.Formula.Core.Parser.Nodes
{
    public class StringLiteral
    {
        public StringLiteral(ParserRuleContext sourceLocation) : base(sourceLocation)
        {

        }

        public NodeKind NodeKind
        {
            get { return NodeKind.StringLiteral; }
        }
    }
}
