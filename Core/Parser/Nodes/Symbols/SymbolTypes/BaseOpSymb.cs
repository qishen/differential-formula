using System;
using System.Collections.Generic;
using System.Diagnostics.Contracts;
using System.Linq;
using System.Text;
using System.Threading;

namespace Microsoft.Formula.Core.Symbols
{ 
    public sealed class BaseOpSymb : Symbol
    {
        private int arity;

        public override SymbolKind Kind
        {
            get { return SymbolKind.BaseOpSymb; }
        }

        public override bool IsSelect
        {
            get
            {
                return OpKind is ReservedOpKind && ((ReservedOpKind)OpKind) == ReservedOpKind.Select;
            }
        }

        public override bool IsTypeUnn
        {
            get
            {
                return OpKind is ReservedOpKind && ((ReservedOpKind)OpKind) == ReservedOpKind.TypeUnn;
            }
        }

        public override bool IsRange
        {
            get
            {
                return OpKind is ReservedOpKind && ((ReservedOpKind)OpKind) == ReservedOpKind.Range;
            }
        }

        public override bool IsRelabel
        {
            get
            {
                return OpKind is ReservedOpKind && ((ReservedOpKind)OpKind) == ReservedOpKind.Relabel;
            }
        }

        public override bool IsReservedOperation
        {
            get { return OpKind is ReservedOpKind; }
        }

        public object OpKind
        {
            get;
            private set;
        }

        public override int Arity
        {
	        get { return arity; }
        }

        public override string PrintableName => throw new NotImplementedException();

    }
}
