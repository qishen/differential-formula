using System;
using System.Collections.Generic;
using System.Text;

using Antlr4.Runtime;

namespace Microsoft.Formula.Core.Parser.Nodes
{
    public interface INode
    {
        ParserRuleContext SourceLocation { get; }
    }
}
