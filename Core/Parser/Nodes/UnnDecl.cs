using System;
using System.Collections.Generic;
using System.Text;

using Antlr4.Runtime;


namespace Microsoft.Formula.Core.Parser.Nodes
{
    public class UnnDecl : Node
    {
        public Id Id { get; }

        public UnnDecl(ParserRuleContext sourceLocation, Id id, List<Node> components) : base(sourceLocation, components)
        {
            Id = id;
        }

        public override NodeKind NodeKind
        {
            get { return NodeKind.UnnDecl; }
        }
    }
}
