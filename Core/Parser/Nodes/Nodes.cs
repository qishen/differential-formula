using System;
using System.Collections.Generic;
using System.Text;

using Antlr4.Runtime;

namespace Microsoft.Formula.Core.Parser.Nodes
{
    public abstract class Nodes : INode
    {

        public ParserRuleContext SourceLocation { get; }

        public HashSet<Node> Components { get; private set; }

        internal Nodes(ParserRuleContext sourceLocation)
        {
            SourceLocation = sourceLocation;
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

        public abstract NodeKind NodeKind
        {
            get;
        }
    }
}
