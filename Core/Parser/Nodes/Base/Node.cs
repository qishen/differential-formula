using System;
using System.Collections.Generic;
using System.Text;

using Antlr4.Runtime;

namespace Microsoft.Formula.Core.Parser.Nodes
{
    public abstract class Node : INode
    {

        public ParserRuleContext SourceLocation { get; }

        public List<Node> Components { get; private set; }

        internal Node(){}

        internal Node(ParserRuleContext sourceLocation)
        {
            SourceLocation = sourceLocation;
            Components = new List<Node>();
        }

        internal Node(ParserRuleContext sourceLocation, List<Node> components)
        {
            SourceLocation = sourceLocation;
            Components = components;
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
