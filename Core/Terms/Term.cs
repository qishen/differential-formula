using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Text;

using Microsoft.Formula.Core.Symbols;

namespace Microsoft.Formula.Core.Terms
{   

    public class Term
    {
        private long uid = -1;
        internal const int FamilyNumeric = 0;
        internal const int FamilyString = 1;
        internal const int FamilyUsrCnst = 2;
        internal const int FamilyApp = 3;

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

        public Term(Symbol symbol, List<Term> args)
        {

        }
    }
}
