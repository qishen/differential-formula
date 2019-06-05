using System;
using System.Collections.Generic;
using System.Text;

using Antlr4.Runtime;

namespace Microsoft.Formula.Core.Parser.Nodes
{
    public class Domain : Node
    {
        // DomainSentences DomainSentences { get; }
        List<Node> DomainSentences { get; }

        string DomainName { get; }

        public Domain(ParserRuleContext sourceLocation, List<Node> domainSentences, string name) : base(sourceLocation)
        {
            DomainSentences = domainSentences;
            DomainName = name;
        }

        public override NodeKind NodeKind
        {
            get { return NodeKind.Domain; }
        }
    }
}
