﻿using System;
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
            });

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
            System.Console.WriteLine(domainSigContext.Id().GetText());

            if(context.domSentences() != null)
            {
                List<Node> sentences = Visit(context.domSentences()) as List<Node>;
                return new Domain(sentences, context);
            }

            return null;
        }

        public override object VisitDomSentences([NotNull] FormulaParser.DomSentencesContext context)
        {
            var sentences = context.domSentence().Select((domSentence) =>
            {
                return Visit(domSentence) as Node;
            });

            return sentences;
        }

        public override object VisitDomConformsExpr([NotNull] FormulaParser.DomConformsExprContext context)
        {
            return base.VisitDomConformsExpr(context);
        }

        public override object VisitDomRuleExpr([NotNull] FormulaParser.DomRuleExprContext context)
        {
            return base.VisitDomRuleExpr(context);
        }

        public override object VisitDomTypeExpr([NotNull] FormulaParser.DomTypeExprContext context)
        {
            return base.VisitDomTypeExpr(context);
        }

        public override object VisitModelSig([NotNull] FormulaParser.ModelSigContext context)
        {
            return base.VisitModelSig(context);
        }

        public override object VisitModel([NotNull] FormulaParser.ModelContext context)
        {
            ModelFactList modFactList = Visit(context.modelFactList()) as ModelFactList;
            return new Model(modFactList, context);
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

        public override object VisitRegularTypeDecl([NotNull] FormulaParser.RegularTypeDeclContext context)
        {
            Id typeId = new Id(context, context.Id().GetText());

            return null;
        }

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

        public override object VisitBinaryExpr([NotNull] FormulaParser.BinaryExprContext context)
        {
            return base.VisitBinaryExpr(context);
        }

        public override object VisitUnaryExpr([NotNull] FormulaParser.UnaryExprContext context)
        {
            return base.VisitUnaryExpr(context);
        }

        public override object VisitFuncCallExpr([NotNull] FormulaParser.FuncCallExprContext context)
        {
            string idStr = context.Id().GetText();
            Id id = new Id(context, idStr);
            List<Term> terms = Visit(context.funcTermList()) as List<Term>;
            Term term = new Term(id, terms, context);
            return term;
        }

        public override object VisitFuncTermList([NotNull] FormulaParser.FuncTermListContext context)
        {
            var terms = context.funcTerm().Select((funcTerm)=>
            {
                Term term = Visit(funcTerm) as Term;
                return term;
            });
            return terms;
        }

        public override object VisitWrappedExpr([NotNull] FormulaParser.WrappedExprContext context)
        {
            return base.VisitWrappedExpr(context);
        }

        public override object VisitModelFactList([NotNull] FormulaParser.ModelFactListContext context)
        {
            ModelFactList modelList = new ModelFactList(context);
            var modelFacts = context.modelFact().Select((modelFact) =>
            {
                Node node = Visit(modelFact) as Node;
                return node;
            });

            foreach (var modelFact in modelFacts)
            {
                modelList.AddComponent(modelFact);
            }

            return modelList;
        }

        public override object VisitModelFact([NotNull] FormulaParser.ModelFactContext context)
        {
            string id = "";
            if(context.Id() != null)
            {
                id = context.Id().GetText();
            }

            Term term = Visit(context.funcTerm()) as Term;
            if(id != "")
            {
                term.Alias = new Id(null, id);
            }         
            return term;
        }

    }
}
