using System;
using System.Collections.Generic;
using System.Text;

using Antlr4.Runtime;

namespace Microsoft.Formula.Core.Parser.Nodes
{
    public class Field : Node
    {   
        public UnnBody UnionBody { get; }

        public Id Id { get; }

        public Field(ParserRuleContext sourceLocation, Id id, UnnBody unnBody) : base(sourceLocation)
        {
            UnionBody = unnBody;
            Id = id;
        }

        public override NodeKind NodeKind
        {
            get { return NodeKind.Field; }
        }
    }
}
