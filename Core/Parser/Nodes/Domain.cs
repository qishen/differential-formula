using System;
using System.Collections.Generic;
using System.Text;

using Antlr4.Runtime;

namespace Microsoft.Formula.Core.Parser.Nodes
{
    public class Domain : Node
    {
        DomainSentences DomainSentences { get; }

        public Domain(DomainSentences domainSentences, ParserRuleContext sourceLocation) : base(sourceLocation)
        {
            DomainSentences = domainSentences;
        }

        public override NodeKind NodeKind
        {
            get { return NodeKind.Domain; }
        }
    }
}
