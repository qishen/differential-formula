using System;
using System.Collections.Generic;
using System.Text;

using Antlr4.Runtime;

namespace Microsoft.Formula.Core.Parser.Nodes
{
    public class Disjunction :Node
    {
        public List<Conjunction> Conjunctions { get; }

        public Disjunction(ParserRuleContext sourceLocation, List<Conjunction> conjunctions) : base(sourceLocation)
        {
            Conjunctions = conjunctions;
        }

        public override NodeKind NodeKind
        {
            get { return NodeKind.Disjunction; }
        }
    }
}
