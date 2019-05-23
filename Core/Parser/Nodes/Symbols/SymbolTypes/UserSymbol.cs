using System;
using System.Collections.Generic;
using System.Diagnostics.Contracts;
using System.Linq;
using System.Text;
using System.Threading;

namespace Microsoft.Formula.Core.Symbols
{ 
    public abstract class UserSymbol : Symbol
    {
        

        /// <summary>
        /// True if this symbol was automatically introduced by the compiler.
        /// </summary>
        public bool IsAutoGen
        {
            get;
            private set;
        }

        /// <summary>
        /// The name of this symbol, excluding namespace qualifications
        /// </summary>
        public string Name
        {
            get;
            private set;
        }

        /// <summary>
        /// The name of this symbol, including namespace qualifications
        /// </summary>
        public string FullName
        {
            get;
            private set;
        }

    
    }
}
