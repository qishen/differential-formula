using System;
using System.Collections.Immutable;
using System.Collections.Generic;
using System.Text;

using Microsoft.Formula.Core.Symbols;
using Antlr4.Runtime;

namespace Microsoft.Formula.Core.Parser.Nodes
{
    // 1. Term can be in the form of termId = TermType(term_1, term_2, ... term_n).
    // 2. Term can be just a constant of basic built-in type.
    // 3. Term can be a variable.
    // 4. Term can be binary arithmetic expression of subterms made of Opkind, Arg1 and Arg2. 
    // For example, (x + 1) * (y - 2) mod 3 in which x, y are variable terms of built-in type.
    //
    // Currently arithmetic operations are not defined for composition terms. Node(1) + Node(2) does not
    // have semantics defined and users should not be allowed to write such expressions.
    public class Term : Node
    {
        private long uid = -1;
        internal const int FamilyNumeric = 0;
        internal const int FamilyString = 1;
        internal const int FamilyUsrCnst = 2;
        internal const int FamilyApp = 3;

        public OpKind Op
        { get; }

        public Id Alias
        {
            get;
        }

        public Id Sig
        {
            get;
        }

        public Cnst Cnst
        {
            get;
        }

        public Groundness Groundness
        {
            get;
            private set;
        }

        public Term(ParserRuleContext sourceLocation, Id alias, Id symbol, List<Node> args) 
            : base(sourceLocation, args)
        {
            Sig = symbol;
            Alias = alias;
            Groundness = Groundness.Type;
        }

        public Term(ParserRuleContext sourceLocation, Cnst cnst) : base(sourceLocation)
        {
            Cnst = cnst;
            Groundness = Groundness.Ground;
        }

        public Term(ParserRuleContext sourceLocation, Id symbol) : base(sourceLocation)
        {
            Sig = symbol;
            Groundness = Groundness.Variable;
        }

        public Term(ParserRuleContext sourceLocation, OpKind op, Term arg1, Term arg2) 
            : base(sourceLocation)
        {
            Op = op;
            this.AddComponent(arg1);
            this.AddComponent(arg2);
        }

        public override NodeKind NodeKind
        {
            get { return NodeKind.Term; }
        }
    }
}
