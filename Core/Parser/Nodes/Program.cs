using System;
using System.Collections.Generic;
using System.Text;

using Antlr4.Runtime;

namespace Microsoft.Formula.Core.Parser.Nodes
{
    public class Program : Node
    {
        public Program(ParserRuleContext sourceLocation, List<Node> moduleList)
            : base(sourceLocation, moduleList)
        {
            
        }

        public override NodeKind NodeKind
        {
            get { return NodeKind.Program; }
        }
    }
}
