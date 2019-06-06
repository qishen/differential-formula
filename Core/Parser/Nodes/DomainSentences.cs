using System;
using System.Collections.Generic;
using System.Text;

using Antlr4.Runtime;

namespace Microsoft.Formula.Core.Parser.Nodes
{
    public class DomainSentences : Node
    {
        public DomainSentences(ParserRuleContext context) : base(context)
        {

        }

        public override NodeKind NodeKind
        {
            get { return NodeKind.DomainSentences; }
        }
    }
}
