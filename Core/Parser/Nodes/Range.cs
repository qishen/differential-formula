using System;
using System.Collections.Generic;
using System.Text;
using System.Numerics;

using Antlr4.Runtime;

namespace Microsoft.Formula.Core.Parser.Nodes
{
    public class Range : Node
    {
        public DecimalLiteral LowerBound { get; }
        public DecimalLiteral UpperBound { get; }

        public Range(ParserRuleContext sourceLocation, string lString, string rString) : base(sourceLocation)
        {
            LowerBound = new DecimalLiteral(sourceLocation, lString);
            UpperBound = new DecimalLiteral(sourceLocation, rString);
        }

        public override NodeKind NodeKind
        {
            get { return NodeKind.Range; }
        }
    }
}
