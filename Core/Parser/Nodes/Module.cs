using System;
using System.Collections.Generic;
using System.Text;

using Antlr4.Runtime;

namespace Microsoft.Formula.Core.Parser.Nodes
{
    public class Module : Node
    {
        public Domain Domain { get; }
        public Model Model { get; }


        public Module(ParserRuleContext sourceLocation, Domain domain) : base(sourceLocation)
        {

        }

        public Module(ParserRuleContext sourceLocation, Model model) : base(sourceLocation)
        {

        }

        public override NodeKind NodeKind
        {
            get { return NodeKind.Module; }
        }
    }
}
