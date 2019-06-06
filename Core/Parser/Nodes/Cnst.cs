using System;
using System.Collections.Generic;
using System.Text;
using System.Numerics;

namespace Microsoft.Formula.Core.Parser.Nodes
{
    public class Cnst : Node
    {
        internal object Raw
        { get; }

        public CnstKind CnstKind
        {
            get;
            private set;
        }
        public Cnst(string s)
        {
            Raw = s;
            CnstKind = CnstKind.String;
        }

        public Cnst(Rational r)
        {
            Raw = r;
            CnstKind = CnstKind.Numeric;
        }

        public Cnst(int i)
        {
            Raw = new Rational(i);
            CnstKind = CnstKind.Numeric;
        }

        public Cnst(BigInteger i)
        {
            Raw = new Rational(i, 1);
            CnstKind = CnstKind.Numeric;
        }

        public override NodeKind NodeKind
        {
            get { return NodeKind.Cnst; }
        }
    }
}
