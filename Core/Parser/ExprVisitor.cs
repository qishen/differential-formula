using System;
using System.Collections.Generic;
using System.Text;

using Antlr4.Runtime;
using Antlr4.Runtime.Misc;
using Antlr4.Runtime.Tree;
using Microsoft.Formula.Core.Parser.Grammar;
using Microsoft.Formula.Core.Parser.Nodes;

namespace Microsoft.Formula.Core.Parser
{
    class TypeDeclVisitor : FormulaBaseVisitor<object>
    {
        private readonly IDictionary<Id, object> typeElems = new Dictionary<Id, object>();

        public override object VisitUnionTypeDecl([NotNull] FormulaParser.UnionTypeDeclContext context)
        {
            Id typeId = new Id(context, context.Id().GetText());
            var unnbodyContext = context.unnBody();
            object node = Visit(unnbodyContext);
            typeElems.Add(typeId, node);
            return node;
        }

        public override object VisitRegularTypeDecl([NotNull] FormulaParser.RegularTypeDeclContext context)
        {
            Id typeId = new Id(context, context.Id().GetText());
            
        }
    }
}
