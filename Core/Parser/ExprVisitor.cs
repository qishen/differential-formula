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

        public override object VisitProgram([NotNull] FormulaParser.ProgramContext context)
        {
            ModuleList moduleList = Visit(context.moduleList()) as ModuleList;
            return new Program(context, moduleList);
        }

        public override object VisitModuleList([NotNull] FormulaParser.ModuleListContext context)
        {
            var moduleList = new ModuleList(context);
            var modules = context.module().Select((module) =>
            {
                return Visit(module);
            }).ToList();

            foreach(var module in modules)
            {
                moduleList.AddComponent(module as Node);
            }

            return moduleList;
        }

        public override object VisitModule([NotNull] FormulaParser.ModuleContext context)
        {
            if(context.domain() != null)
            {
                Domain domain = Visit(context.domain()) as Domain;
                return new Module(context, domain);
            }
            else if(context.model() != null)
            {
                Model model = Visit(context.model()) as Model;
                return new Module(context, model);
            }

            return null;
        }

        public override object VisitDomain([NotNull] FormulaParser.DomainContext context)
        {
            var domainSigContext = context.domainSig();
            string domainName = domainSigContext.Id().GetText();
            List<Node> sentences = null;
            if(context.domSentences() != null)
            {
                sentences = Visit(context.domSentences()) as List<Node>;
            }
            return new Domain(context, sentences, domainName);
        }

        public override object VisitDomSentences([NotNull] FormulaParser.DomSentencesContext context)
        {
            var contextList = context.domSentence();

            var sentences = context.domSentence().Select((domSentence) =>
            {
                System.Console.WriteLine(domSentence.GetText());
                return Visit(domSentence);
            }).ToList();

            return sentences;                                                                                                            
        }

        /*
        public override object VisitDomConformsExpr([NotNull] FormulaParser.DomConformsExprContext context)
        {
            return base.VisitDomConformsExpr(context);
        }

        public override object VisitDomRuleExpr([NotNull] FormulaParser.DomRuleExprContext context)
        {
            return base.VisitDomRuleExpr(context);
        }
        */

        public override object VisitDomTypeExpr([NotNull] FormulaParser.DomTypeExprContext context)
        {
            return Visit(context.typeDecl());
        }

        public override object VisitModelSig([NotNull] FormulaParser.ModelSigContext context)
        {
            return base.VisitModelSig(context);
        }

        public override object VisitUnionTypeDecl([NotNull] FormulaParser.UnionTypeDeclContext context)
        {
            Id typeId = new Id(context, context.Id().GetText());
            var unnbodyContext = context.unnBody();
            object node = Visit(unnbodyContext);
            // Map Id name to Union type node.
            typeElems.Add(context.Id().GetText(), node);
            return node;
        }

        /*
        public override object VisitRegularTypeDecl([NotNull] FormulaParser.RegularTypeDeclContext context)
        {
            Id typeId = new Id(context, context.Id().GetText());

            return null;
        }
        */
        public override object VisitFields([NotNull] FormulaParser.FieldsContext context)
        {
            var fields = context.field().Select((field) =>
            {
                return (Node)Visit(field);
            });

            var fieldsNode = new Fields(context);
            foreach (var field in fields)
            {
                fieldsNode.AddComponent(field);
            }

            return fieldsNode;
        }

        public override object VisitField([NotNull] FormulaParser.FieldContext context)
        {
            Id id = null; 
        
            if (context.Id() != null)
            {
                id = (Id)Visit(context.Id());
            }

            if (context.ANY() != null)
            {

            }

            UnnBody body = (UnnBody)Visit(context.unnBody());

            return new Field(context, id, body);
        }

        /*
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
        */

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

        /*
        public override object VisitPrimitiveExpr([NotNull] FormulaParser.PrimitiveExprContext context)
        {
            if (context.atom().Id() != null)
            {
                return new Id(context.atom(), context.atom().Id().GetText());
            }
            else
            {
                return null;
            }
        }
        */
        
        

    }
}
