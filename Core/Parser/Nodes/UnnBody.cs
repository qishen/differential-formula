using System;
using System.Collections.Generic;
using System.Text;

using Antlr4.Runtime;


namespace Microsoft.Formula.Core.Parser.Nodes
{
    public class UnnBody : Nodes
    {
        public Id Name { get; private set; }

        public int ChildCount
        {
            get { return Components.Count; }
        }

        public UnnBody(ParserRuleContext sourceLocation) : base(sourceLocation)
        {

        }

        public override NodeKind NodeKind
        {
            get { return NodeKind.UnionBody; }
        }
    }
}
