using System;
using System.Collections.Generic;
using System.Text;
using System.Numerics;

using Antlr4.Runtime;

namespace Microsoft.Formula.Core.Parser.Nodes
{
    public class DecimalLiteral : Node
    {
        public BigInteger Value { get; }

        public DecimalLiteral(ParserRuleContext sourceLocation, string integerString) : base(sourceLocation)
        {
            Value = BigInteger.Parse(integerString);
        }

        public override NodeKind NodeKind
        {
            get { return NodeKind.DecimalLiteral; }
        }
    }
}
