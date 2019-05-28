using System;
using System.Collections.Generic;
using System.Text;

using Antlr4.Runtime;

namespace Microsoft.Formula.Core.Parser.Nodes
{
    public class Model : Node
    {
        public ModelFactList ModelFactList { get; }

        public Model(ModelFactList modelFactList, ParserRuleContext sourceLocation) : base(sourceLocation)
        {
            ModelFactList = modelFactList;
        }

        public override NodeKind NodeKind
        {
            get { return NodeKind.Model; }
        }
    }
}
