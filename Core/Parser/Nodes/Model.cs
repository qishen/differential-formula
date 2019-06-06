using System;
using System.Collections.Generic;
using System.Text;

using Antlr4.Runtime;

namespace Microsoft.Formula.Core.Parser.Nodes
{
    public class Model : Node
    {   
        public bool IsPartial { get; }

        public string ModelName { get; }

        public List<Term> Terms { get; }

        public List<Model> ModRefs { get; }

        public Model(ParserRuleContext sourceLocation, bool isPartial, string modelName, List<Node> terms) 
            : base(sourceLocation, terms)
        {
            IsPartial = isPartial;
            ModelName = modelName;
            ModRefs = new List<Model>();
        }

        public bool AddModRef(Model model)
        {
            if (model.ModelName != this.ModelName)
            {
                ModRefs.Add(model);
                return true;
            }
            else
            {
                return false;
            }
        }

        public override NodeKind NodeKind
        {
            get { return NodeKind.Model; }
        }
    }
}
