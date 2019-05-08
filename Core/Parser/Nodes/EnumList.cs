using System;
using System.Collections.Generic;
using System.Text;

using Antlr4.Runtime;

namespace Microsoft.Formula.Core.Parser.Nodes
{
    public class EnumList : Node
    {
        public HashSet<Node> Components { get; }

        public EnumList(ParserRuleContext sourceLocation) : base(sourceLocation)
        {

        }

        public bool AddComponent(Node node)
        {
            if (!Components.Contains(node))
            {
                Components.Add(node);
                return true;
            }
            else
            {
                return false;
            }
        }

        public override NodeKind NodeKind
        {
            get { return NodeKind.EnumList; }
        }
    }
}
