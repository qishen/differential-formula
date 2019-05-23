using System;
using System.Collections.Generic;
using System.Diagnostics.Contracts;
using System.Linq;
using System.Text;
using System.Threading;

namespace Microsoft.Formula.Core.Symbols
{
    public sealed class BaseSortSymb : Symbol
    {
        public override SymbolKind Kind
        {
            get { return SymbolKind.BaseSortSymb; }
        }

        public override int Arity
        {
            get { return 0; }
        }

        public BaseSortKind SortKind
        {
            get;
            private set;
        }      
       
        internal BaseSortSymb(BaseSortKind sortKind)      
        {
            SortKind = sortKind;
        }

        public override string PrintableName => throw new NotImplementedException();
    }
}
