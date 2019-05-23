using System;
using System.Collections.Generic;
using System.Text;

namespace Microsoft.Formula.Core.Parser.Nodes.Base
{
    public abstract class NodeType
    {
        private string TypeName { get; }
        private int Index { get; }

        private NodeType BaseType;

        protected NodeType(string s, int index)
        {
            TypeName = s;
            Index = index;
        }
    }
}
