using System;
using System.Collections.Immutable;
using System.Collections.Generic;
using System.Text;

using Microsoft.Formula.Core.Symbols;
using Antlr4.Runtime;

namespace Microsoft.Formula.Core.Parser.Nodes
{
    public class Term : Nodes
    {
        private long uid = -1;
        internal const int FamilyNumeric = 0;
        internal const int FamilyString = 1;
        internal const int FamilyUsrCnst = 2;
        internal const int FamilyApp = 3;

        public Id Alias
        {
            get;
            set;
        }

        public Id Sig
        {
            get;
            set;
        }

        public Groundness Groundness
        {
            get;
            private set;
        }

        public Symbol Symbol
        {
            get;
            private set;
        }

        public ImmutableArray<Term> Args
        {
            get;
            private set;
        }

        public Term(Symbol symbol, List<Term> args, ParserRuleContext sourceLocation) : base(sourceLocation)
        {

        }

        public Term(Id symbol, List<Term> args, ParserRuleContext sourceLocation) : base(sourceLocation)
        {
            Sig = symbol;
            Args = ImmutableArray.ToImmutableArray(args);
        }

        public Term(ParserRuleContext sourceLocation) : base(sourceLocation)
        {

        }

        public override NodeKind NodeKind => throw new NotImplementedException();
    }
}
