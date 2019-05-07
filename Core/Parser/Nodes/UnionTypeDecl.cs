using System;
using System.Collections.Generic;
using System.Text;

using Antlr4.Runtime;


namespace Microsoft.Formula.Core.Parser.Nodes
{
    public class UnionTypeDecl : Node
    {
        public Id Id { get; }
        public UnnBody UnnBody { get; }

        public UnionTypeDecl(ParserRuleContext sourceLocation, Id id, UnnBody unnBody) : base(sourceLocation)
        {
            Id = id;
            UnnBody = unnBody;
        }

        public override NodeKind NodeKind
        {
            get { return NodeKind.TypeDeclExpr; }
        }
    }
}
