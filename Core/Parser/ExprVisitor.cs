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
            var moduleList = Visit(context.moduleList()) as List<Node>;
            return new Program(context, moduleList);
        }

        public override object VisitModuleList([NotNull] FormulaParser.ModuleListContext context)
        {
            var modules = context.module().Select((module) => {
                return (Node)Visit(module);
            }).ToList();
            return modules;
        }

        public override object VisitModule([NotNull] FormulaParser.ModuleContext context)
        {
            if (context.domain() != null)
            {
                Domain domain = (Domain)Visit(context.domain());
                return domain;
            }
            else if (context.model() != null)
            {
                Model model = (Model)Visit(context.model());
                return model;
            }
            else
            {
                throw new ParseCanceledException();
            }
        }

        public override object VisitDomain([NotNull] FormulaParser.DomainContext context)
        {
            var domainSigContext = context.domainSig();
            string domainName = domainSigContext.Id().GetText();
            var sentences = (List<Node>)Visit(context.domSentences());         
            return new Domain(context, sentences, domainName);
        }

        public override object VisitModel([NotNull] FormulaParser.ModelContext context)
        {
            // TODO: Add domain references with includes and excludes keyword.
            // modelIntro.modRef()
            var modelSig = context.modelSig();
            var modelIntro = modelSig.modelIntro();
            string modelName = modelIntro.Id().GetText();
            bool isPartial = modelSig.modelIntro().PARTIAL() != null;

            var termsList = context.modelFactList().Select((modelFactList) => {
                List<Node> terms = Visit(modelFactList) as List<Node>;
                return terms;
            });

            List<Node> allTerms = new List<Node>();
            foreach (List<Node> terms in termsList)
            {
                foreach (Node term in terms)
                {
                    allTerms.Add(term);
                }
            }

            return new Model(context, isPartial, modelName, allTerms);
        }

        public override object VisitModRef([NotNull] FormulaParser.ModRefContext context)
        {
            return base.VisitModRef(context);
        }

        public override object VisitModelFactList([NotNull] FormulaParser.ModelFactListContext context)
        {
            List<Node> terms = context.funcTerm().Select((funcTerm) => {
                return (Node)Visit(funcTerm);
            }).ToList();
            return terms;
        }

        public override object VisitAtomTerm([NotNull] FormulaParser.AtomTermContext context)
        {
            if (context.atom().Id() != null)
            {
                string id = context.atom().Id().GetText();
                Cnst cnst = new Cnst(id);
                return new Term(context.atom(), cnst);
            }
            else if (context.atom().constant() != null)
            {
                Cnst cnst = Visit(context.atom().constant()) as Cnst;
                return new Term(context.atom(), cnst);
            }
            else
            {
                throw new ParseCanceledException();
            }
        }

        public override object VisitCompositionTerm([NotNull] FormulaParser.CompositionTermContext context)
        {
            Id alias = null;
            Id typeName;
            if (context.Id(0) != null && context.Id(1) != null)
            {
                alias = new Id(context, context.Id(0).GetText());
                typeName = new Id(context, context.Id(1).GetText());
            }
            else
            {
                typeName = new Id(context, context.Id(0).GetText());
            }
            
            var terms = (List<Node>)Visit(context.funcTermList());
            return new Term(context, alias, typeName, terms);
        }

        public override object VisitFuncTermList([NotNull] FormulaParser.FuncTermListContext context)
        {
            var terms = context.funcTerm().Select((funcTerm) => {
                return (Node)Visit(funcTerm);
            }).ToList();
            return terms;
        }

        public override object VisitConstant([NotNull] FormulaParser.ConstantContext context)
        {
            if (context.DECIMAL() != null)
            {
                Rational r;
                Rational.TryParseDecimal(context.DECIMAL().GetText(), out r);
                return new Cnst(r);
            }
            else if (context.REAL() != null)
            {
                Rational r;
                Rational.TryParseDecimal(context.REAL().GetText(), out r);
                return new Cnst(r);
            }
            else if (context.FRAC() != null)
            {
                Rational r;
                Rational.TryParseFraction(context.FRAC().GetText(), out r);
                return new Cnst(r);
            }
            else if (context.STRING() != null)
            {
                return new Cnst(context.STRING().GetText());
            }
            else
            {
                throw new ParseCanceledException();
            }
        }

        public override object VisitModelSig([NotNull] FormulaParser.ModelSigContext context)
        {
            return base.VisitModelSig(context);
        }

        public override object VisitDomSentences([NotNull] FormulaParser.DomSentencesContext context)
        {
            var sentences = context.domSentence().Select((domSentence) => {
                return (Node)Visit(domSentence);
            }).ToList();
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
            var typeExpr = Visit(context.typeDecl());
            return typeExpr;
        }

        public override object VisitUnionTypeDecl([NotNull] FormulaParser.UnionTypeDeclContext context)
        {
            Id unionTypeName = new Id(context, context.Id().GetText());         
            List<Node> components = Visit(context.unnBody()) as List<Node>;
            UnnDecl unionNode = new UnnDecl(context, unionTypeName, components);
            // Map Id name to Union type node.
            typeElems.Add(unionTypeName.Name, unionNode);
            return unionNode;
        }

        public override object VisitRegularTypeDecl([NotNull] FormulaParser.RegularTypeDeclContext context)
        {
            Id typeId = new Id(context, context.Id().GetText());
            List<Node> fields = Visit(context.fields()) as List<Node>;
            ConDecl conDecl = new ConDecl(context, typeId, fields);
            return conDecl;
        }

        public override object VisitFields([NotNull] FormulaParser.FieldsContext context)
        {
            var fields = context.field().Select((field) => {
                return (Node)Visit(field);
            }).ToList();
            return fields;
        }

        public override object VisitField([NotNull] FormulaParser.FieldContext context)
        {
            string label = null;
            if (context.Id(0) != null)
            {
                label = context.Id(0).GetText();
            }

            bool isAny = false;
            if (context.ANY() != null)
            {
                isAny = context.ANY() != null;
            }

            if (context.unnBody() != null)
            {
                var body = Visit(context.unnBody()) as List<Node>;
                UnnDecl unionNode = new UnnDecl(context.unnBody(), null, body);
                return new Field(context, label, unionNode, isAny);
            }
            else if (context.Id(1) != null)
            {
                string typeName = context.Id(1).GetText();
                Id typeNode = new Id(context, typeName);
                return new Field(context, label, typeNode, false);
            }
            else
            {
                throw new ParseCanceledException();
            }          
        }

        public override object VisitUnnBody([NotNull] FormulaParser.UnnBodyContext context)
        {
            var unnElems = context.unnElem().Select((unnElem) => {
                return (Node)Visit(unnElem);
            }).ToList();
            return unnElems;
        }
        
        public override object VisitUnnElem([NotNull] FormulaParser.UnnElemContext context)
        {
            Debug.Assert(!(context.Id() == null && context.enumList() == null));

            if (context.Id() != null)
            {
                string subtypeName = context.Id().GetText();
                return new Id(context, subtypeName);
            }
            else if (context.enumList() != null)
            {
                return Visit(context.enumList());
            }
            else
            {
                throw new ParseCanceledException();
            }
        }

        public override object VisitEnumList([NotNull] FormulaParser.EnumListContext context)
        {
            var enumList = context.enumCnst().Select((enumCnst) => {
                return Visit(enumCnst) as Node;
            }).ToList();
            return new EnumList(context, enumList);
        }

        public override object VisitEnumCnst([NotNull] FormulaParser.EnumCnstContext context)
        {
            if (context.RANGE() != null)
            {
                var lowStr = context.DECIMAL(0).GetText();
                var highStr = context.DECIMAL(1).GetText();
                return new Range(context, lowStr, highStr);
            }
            else if (context.constant() != null)
            {
                return Visit(context.constant());
            }
            else
            {
                throw new ParseCanceledException();
            }
        }

    }
}
