using System;
using System.Collections.Immutable;
using System.Collections.Generic;
using System.Text;

using Microsoft.Formula.Core.Symbols;
using Antlr4.Runtime;

namespace Microsoft.Formula.Core.Parser.Nodes
{
    // 1. Term can be in composite form as termId = TermType(term_1, term_2, ... term_n).
    // 2. Term can be just a constant of basic built-in type.
    // 3. Term can be a variable.
    // 4. Term can be binary arithmetic expression of subterms made of Opkind, Arg1 and Arg2. 
    // For example, (x + 1) * (y - 2) mod 3 in which x, y are variable terms of built-in type.
    //
    // Currently arithmetic operations are not defined for composition terms. Node(1) + Node(2) does not
    // have semantics defined and users should not be allowed to write such expressions.
    public class Term : Node
    {
        public Groundness Groundness
        {
            get;
            private set;
        }

        // Nullable type for OpKind
        // operator for binary arithmetic term.
        public OpKind? Op
        {
            get;
        }

        public Id Alias
        {
            get;
            set;
        }

        // The name of the constructor of composite term.
        public Id Sig
        {
            get;
        }

        // Use an Id node to represent variable.
        public Id Variable
        {
            get;
        }

        // A node to hold constant value if the term is a constant.
        public Cnst Cnst
        {
            get;
        }

        public bool IsArithmeticTerm
        {
            get { return Op.HasValue; }
        }


        // Constructor without parser context.
        public Term(Id alias, Id symbol, List<Node> args)
        {
            Sig = symbol;
            Alias = alias;
            Groundness = Groundness.Composite;
        }


        // Constructor for compositional term in which each argument is also a term.
        public Term(ParserRuleContext sourceLocation, Id alias, Id symbol, List<Node> args) 
            : base(sourceLocation, args)
        {
            Sig = symbol;
            Alias = alias;
            Groundness = Groundness.Composite;
        }


        // Constructor for constant term
        public Term(ParserRuleContext sourceLocation, Cnst cnst) : base(sourceLocation)
        {
            Cnst = cnst;
            Groundness = Groundness.Ground;
        }


        // Constructor for variable term
        public Term(ParserRuleContext sourceLocation, Id symbol) : base(sourceLocation)
        {
            Variable = symbol;
            Groundness = Groundness.Variable;
        }


        // Binary arithmetic term that contains two arguments and an operator.
        // For example, arithmetic expression inside Node(x*3) can be written as <*>(x, 3),
        // so an arithmetic term is also an composite term in infix form.
        public Term(ParserRuleContext sourceLocation, OpKind op, Node arg1, Node arg2) 
            : base(sourceLocation)
        {
            Op = (OpKind?) op;
            this.AddComponent(arg1);
            this.AddComponent(arg2);
            Groundness = Groundness.Composite;
        }


        public void AddTermAlias(Id id)
        {
            Alias = id;
        }


        public override NodeKind NodeKind
        {
            get { return NodeKind.Term; }
        }
    }
}
