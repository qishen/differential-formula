using System;
using System.Collections.Generic;
using System.Diagnostics.Contracts;
using System.Linq;
using System.Text;
using System.Threading;


namespace Microsoft.Formula.Core.Symbols
{
    public sealed class MapSymb : UserSymbol
    {
        private int arity;
        private Tuple<bool, string>[] domAttrs;
        private Tuple<bool, string>[] codAttrs;
        

        public override int Arity
        {
            get { return arity; }
        }

        public int DomArity
        {
            get { return domAttrs.Length; }
        }

        public int CodArity
        {
            get { return codAttrs.Length; }
        }

        public override SymbolKind Kind
        {
            get { return SymbolKind.MapSymb; }
        }

        public override string PrintableName => throw new NotImplementedException();
    }
}
