using System;
using System.Collections.Generic;
using System.Diagnostics.Contracts;
using System.Linq;
using System.Text;
using System.Threading;

namespace Microsoft.Formula.Core.Symbols
{
    public sealed class ConSymb : UserSymbol
    {
        private int arity;
        private Tuple<bool, string>[] fldAttrs;
       

        public override int Arity
        {
            get { return arity; }
        }

        public override SymbolKind Kind
        {
            get { return SymbolKind.ConSymb; }
        }

        public override string PrintableName => throw new NotImplementedException();
    }
}
