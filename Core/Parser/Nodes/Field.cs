using System;
using System.Collections.Generic;
using System.Text;

using Antlr4.Runtime;

namespace Microsoft.Formula.Core.Parser.Nodes
{
    public class Field : Node
    {   
        public Field(ParserRuleContext sourceLocation, string label, Node type, bool isAny) 
            : base(sourceLocation)
        {
            Label = label;
            Type = type;
            IsAny = isAny;
        }

        public bool IsAny
        {
            get;
            private set;
        }
        
        public string Label
        {
            get;
            private set;
        }

        public Node Type
        {
            get;
            private set;
        }

        public override NodeKind NodeKind
        {
            get { return NodeKind.Field; }
        }
    }
}
