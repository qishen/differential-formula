using System;
using System.Collections.Generic;
using System.Text;

using Antlr4.Runtime;

namespace Microsoft.Formula.Core.Parser.Nodes
{
    public class Program : Node
    {
        public ModuleList ModuleList { get; }

        public Program(ParserRuleContext sourceLocation, ModuleList moduleList)
        {
            ModuleList = moduleList;
        }

        public override NodeKind NodeKind
        {
            get { return NodeKind.Program; }
        }
    }
}
