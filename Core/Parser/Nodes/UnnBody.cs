using System;
using System.Collections.Generic;
using System.Text;

using Antlr4.Runtime;


namespace Microsoft.Formula.Core.Parser.Nodes
{
    public class UnnBody : Node
    {
        public Id Name { get; private set; }

        public HashSet<Node> Components { get; private set; }

        public int ChildCount
        {
            get { return Components.Count; }
        }

        public UnnBody(ParserRuleContext sourceLocation) : base(sourceLocation)
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
            get { return NodeKind.Union; }
        }

        public override bool Equals(object obj)
        {
            return base.Equals(obj);
        }
    }
}
