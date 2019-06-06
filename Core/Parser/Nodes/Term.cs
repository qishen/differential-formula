using System;
using System.Collections.Immutable;
using System.Collections.Generic;
using System.Text;

using Microsoft.Formula.Core.Symbols;
using Antlr4.Runtime;

namespace Microsoft.Formula.Core.Parser.Nodes
{
    public class Term : Node
    {
        private long uid = -1;
        internal const int FamilyNumeric = 0;
        internal const int FamilyString = 1;
        internal const int FamilyUsrCnst = 2;
        internal const int FamilyApp = 3;

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

        public Term(ParserRuleContext sourceLocation, Cnst cnst)
        {
            Cnst = cnst;
            Groundness = Groundness.Ground;
        }

        public Term(ParserRuleContext sourceLocation, Id symbol) : base(sourceLocation)
        {
            Sig = symbol;
            Groundness = Groundness.Variable;
        }

        public override NodeKind NodeKind
        {
            get { return NodeKind.Term; }
        }
    }
}
