using System;
using System.Collections.Generic;
using System.Text;
using System.Collections.Generic;

using Antlr4.Runtime;

namespace Microsoft.Formula.Core.Parser.Nodes.Base
{
    public interface INode
    {   
        INode Parent { get; }
        List<INode> Children { get; }

        ParserRuleContext SourceLocation { get; }

        bool Contains(INode other);
        
    }
}
