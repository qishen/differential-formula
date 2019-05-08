using System;
using System.Linq;
using System.Collections.Generic;
using System.Text;
using System.Diagnostics;

using Antlr4.Runtime;
using Antlr4.Runtime.Misc;
using Antlr4.Runtime.Tree;
using Microsoft.Formula.Core.Parser.Grammar;
using Microsoft.Formula.Core.Parser.Nodes;

namespace Microsoft.Formula.Core.Parser
{
    public class ExprVisitor : FormulaBaseVisitor<object>
    {
        private readonly IDictionary<string, object> typeElems = new Dictionary<string, object>();

        public override object VisitUnionTypeDecl([NotNull] FormulaParser.UnionTypeDeclContext context)
        {
            Id typeId = new Id(context, context.Id().GetText());
            var unnbodyContext = context.unnBody();
            object node = Visit(unnbodyContext);
            // Map Id name to Union type node.
            typeElems.Add(context.Id().GetText(), node);
            return node;
        }

        public override object VisitRegularTypeDecl([NotNull] FormulaParser.RegularTypeDeclContext context)
        {
            Id typeId = new Id(context, context.Id().GetText());

            return null;
        }

        public override object VisitUnnBody([NotNull] FormulaParser.UnnBodyContext context)
        {
            var unnBody = new UnnBody(context);
            var unnElems = context.unnElem().Select((unnElem) =>
            {
                return (Node)Visit(unnElem);
            }
            );

            foreach (var unnElem in unnElems)
            {
                unnBody.AddComponent(unnElem);
            }

            return unnBody;
        }

        public override object VisitUnnElem([NotNull] FormulaParser.UnnElemContext context)
        {
            Debug.Assert(!(context.Id() == null && context.enumList() == null));
            if (context.Id() != null)
            {
                return new Id(context, context.Id().GetText());
            }
            else if (context.enumList() != null)
            {
                return new EnumList(context);
            }
            else
            {
                return null;
            }
        }

        public override object VisitEnumList([NotNull] FormulaParser.EnumListContext context)
        {
            return base.VisitEnumList(context);
        }

        public override object VisitEnumCnst([NotNull] FormulaParser.EnumCnstContext context)
        {
            if (context.RANGE() != null)
            {
                var left = Visit(context.DECIMAL(0));
                var right = Visit(context.DECIMAL(1));
            }
            else if (context.DECIMAL() != null)
            {
                return new DecimalLiteral(context, context.DECIMAL().ToString());
            }
            else if (context.REAL() != null)
            {

            }
            else if (context.FRAC() != null)
            {

            }
            else if (context.STRING() != null)
            {

            }

            return null;
        }

    }
}
