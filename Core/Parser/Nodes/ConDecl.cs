using System;
using System.Collections.Generic;
using System.Text;

using Antlr4.Runtime;

namespace Microsoft.Formula.Core.Parser.Nodes
{
    public class ConDecl : Node
    {
        public ConDecl(ParserRuleContext sourceLocation, Id typeName, List<Node> fields) 
            : base(sourceLocation, fields)
        {
            Id = typeName;
        }

        public Id Id
        { get; }

        public override NodeKind NodeKind
        {
            get { return NodeKind.TypeDecl; }
        }
    }
}
