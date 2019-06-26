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

        public static RelKind Str2RelKind(string s)
        {
            RelKind op;

            switch (s)
            {
                case ">":
                    op = RelKind.Gt;
                    break;
                case ">=":
                    op = RelKind.Ge;
                    break;
                case "<":
                    op = RelKind.Lt;
                    break;
                case "<=":
                    op = RelKind.Le;
                    break;
                case "=":
                    op = RelKind.Eq;
                    break;
                case "no":
                    op = RelKind.No;
                    break;
                case "!=":
                    op = RelKind.Neq;
                    break;
                case ":":
                    op = RelKind.Typ;
                    break;
                default:
                    op = RelKind.Null;
                    break;
            }

            return op;
        }

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
            List<Node> terms = context.modelFact().Select((modelFact) => {
                return (Node)Visit(modelFact);
            }).ToList();
            return terms;
        }

        public override object VisitCompositionalTerm([NotNull] FormulaParser.CompositionalTermContext context)
        {
            Id alias = new Id(context, context.Id().GetText());

            Term composTerm = Visit(context.compositionalTermWithoutAlias()) as Term;
            composTerm.AddTermAlias(alias);
            return composTerm;
        }

        public override object VisitNestedCompositionalTerm([NotNull] FormulaParser.NestedCompositionalTermContext context)
        {
            Id typeName = new Id(context, context.Id().GetText());
            var terms = context.compositionalTermWithoutAlias().Select((term) => {
                return (Node)Visit(term);
            }).ToList();
            return new Term(context, null, typeName, terms);
        }

        public override object VisitNonNestedCompositionalTerm([NotNull] FormulaParser.NonNestedCompositionalTermContext context)
        {
            Id typeName = new Id(context, context.Id().GetText());
            var terms = context.atom().Select((atom) => {
                return (Node)Visit(atom);
            }).ToList();
            return new Term(context, null, typeName, terms);
        }

        public override object VisitTerms([NotNull] FormulaParser.TermsContext context)
        {
            var terms = context.term().Select((term) => {
                return (Node)Visit(term);
            }).ToList();
            return terms;
        }

        public override object VisitTerm([NotNull] FormulaParser.TermContext context)
        {
            if (context.atom() != null)
            {
                return Visit(context.atom());
            }
            else if (context.compositionalTermWithoutAlias() != null)
            {
                return Visit(context.compositionalTermWithoutAlias());
            }
            else if (context.arithmeticTerm() != null)
            {
                return Visit(context.arithmeticTerm());
            }
            else
            {
                throw new ParseCanceledException();
            }
        }

        public override object VisitAtom([NotNull] FormulaParser.AtomContext context)
        {
            if (context.Id() != null)
            {
                string idStr = context.Id().GetText();
                Id id = new Id(context, idStr);
                return new Term(context, id);
            }
            else if (context.constant() != null)
            {
                Cnst cnst = Visit(context.constant()) as Cnst;
                return new Term(context, cnst);
            }
            else
            {
                throw new ParseCanceledException();
            }
        }

        public override object VisitAtomTerm([NotNull] FormulaParser.AtomTermContext context)
        {
            return Visit(context.atom()) as Term;
        }

        public override object VisitAddSubArithTerm([NotNull] FormulaParser.AddSubArithTermContext context)
        {
            var term1 = Visit(context.arithmeticTerm(0)) as Term;
            var term2 = Visit(context.arithmeticTerm(1)) as Term;
            Term binaryTerm;
            if (context.PLUS() != null)
            {
                binaryTerm = new Term(context, OpKind.Add, term1, term2);
            }
            else
            {
                binaryTerm = new Term(context, OpKind.Sub, term1, term2);
            }

            return binaryTerm;
        }

        public override object VisitModArithTerm([NotNull] FormulaParser.ModArithTermContext context)
        {
            var term1 = Visit(context.arithmeticTerm(0)) as Term;
            var term2 = Visit(context.arithmeticTerm(1)) as Term;
            return new Term(context, OpKind.Mod, term1, term2);
        }

        public override object VisitMulDivArithTerm([NotNull] FormulaParser.MulDivArithTermContext context)
        {
            var term1 = Visit(context.arithmeticTerm(0)) as Term;
            var term2 = Visit(context.arithmeticTerm(1)) as Term;
            Term binaryTerm;
            if (context.MUL() != null)
            {
                binaryTerm = new Term(context, OpKind.Mul, term1, term2);
            }
            else
            {
                binaryTerm = new Term(context, OpKind.Div, term1, term2);
            }

            return binaryTerm;
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

        public override object VisitFormulaRule([NotNull] FormulaParser.FormulaRuleContext context)
        {
            List<Node> head = Visit(context.terms()) as List<Node>;
            Disjunction body = Visit(context.disjunction()) as Disjunction;
            return new Rule(context, head, body);
        }

        public override object VisitDisjunction([NotNull] FormulaParser.DisjunctionContext context)
        {
            var conjunctions = context.conjunction().Select((conjunction) => {
                return Visit(conjunction) as Conjunction;
            }).ToList();

            return new Disjunction(context, conjunctions);
        }

        public override object VisitConjunction([NotNull] FormulaParser.ConjunctionContext context)
        {
            var constraints = context.constraint().Select((constraint) => {
                return Visit(constraint) as Node;
            }).ToList();

            return new Conjunction(context, constraints);
        }

        public override object VisitPredConstraint([NotNull] FormulaParser.PredConstraintContext context)
        {
            bool negated;
            if(context.NO() != null)
            {
                negated = true;
            }
            else
            {
                negated = false;
            }
            Term term = Visit(context.compositionalTermWithoutAlias()) as Term;
            return new Constraint(context.compositionalTermWithoutAlias(), negated, term);
        }

        public override object VisitAggregationCountConstraint([NotNull] FormulaParser.AggregationCountConstraintContext context)
        {
            SetComprehension compr = Visit(context.setComprehension()) as SetComprehension;
            Rational r;
            Rational.TryParseDecimal(context.DECIMAL().GetText(), out r);
            Cnst num = new Cnst(r);
            string opStr = context.relOp().GetText();
            RelKind op = Str2RelKind(opStr);
            return new Constraint(context, op, compr, num);
        }

        public override object VisitBinaryConstraint([NotNull] FormulaParser.BinaryConstraintContext context)
        {
            Node arg1 = Visit(context.arithmeticTerm(0)) as Node;
            Node arg2 = Visit(context.arithmeticTerm(1)) as Node;
            string opStr = context.relOp().GetText();
            RelKind op = Str2RelKind(opStr);
            return new Constraint(context, op, arg1, arg2);
        }

        public override object VisitTypeConstraint([NotNull] FormulaParser.TypeConstraintContext context)
        {
            string id = context.Id(0).GetText();
            string typeStr = context.Id(1).GetText();
            Id variable = new Id(context, id);
            Id typeName = new Id(context, typeStr);
            return new Constraint(context, RelKind.Typ, variable, typeName);
        }

        public override object VisitVariableBindingConstraint([NotNull] FormulaParser.VariableBindingConstraintContext context)
        {
            string id = context.Id().GetText();
            Id variable = new Id(context, id);
            Term term = Visit(context.compositionalTermWithoutAlias()) as Term;
            return new Constraint(context, RelKind.Eq, variable, term);
        }

        public override object VisitDerivedConstantConstraint([NotNull] FormulaParser.DerivedConstantConstraintContext context)
        {
            bool negated;
            if (context.NO() != null)
            {
                negated = true;
            }
            else
            {
                negated = false;
            }

            Id variable = new Id(context, context.Id().GetText());
            return new Constraint(context, negated, variable);
        }

        public override object VisitSetComprehension([NotNull] FormulaParser.SetComprehensionContext context)
        {
            return base.VisitSetComprehension(context);
        }

    }
}
