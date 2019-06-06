using System;
using System.Collections.Generic;
using System.Text;
using System.Numerics;

using Antlr4.Runtime;

namespace Microsoft.Formula.Core.Parser.Nodes
{
    public class Range : Node
    {
        public Rational LowerBound { get; }
        public Rational UpperBound { get; }

        public Range(ParserRuleContext sourceLocation, string lString, string rString) : base(sourceLocation)
        {
            Rational lowerBound, upperBound;
            Rational.TryParseDecimal(lString, out lowerBound);
            Rational.TryParseDecimal(rString, out upperBound);
            LowerBound = lowerBound;
            UpperBound = upperBound;
        }

        public override NodeKind NodeKind
        {
            get { return NodeKind.Range; }
        }

        public List<Rational> GetRationals()
        {
            throw new NotImplementedException();
        }
    }
}
