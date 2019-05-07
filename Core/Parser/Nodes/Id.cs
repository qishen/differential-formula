using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Text;

using Antlr4.Runtime;

namespace Microsoft.Formula.Core.Parser.Nodes
{
    public class Id : Node
    {
        private static readonly char[] splitChars = new char[] { '.' };

        public string Name { get; private set; }

        public ImmutableArray<string> Fragments { get; private set; }

        public Id(ParserRuleContext sourceLocation, string name) : base(sourceLocation)
        {
            Name = name;
            Fragments = ImmutableArray.ToImmutableArray(Name.Split(splitChars));
        }
        
        public override NodeKind NodeKind
        {
            get { return NodeKind.Id; }
        }

        public bool HasFragments()
        {
            return Fragments.Length > 1;
        }

        public override bool Equals(object obj)
        {
            if (obj == null || !this.GetType().Equals(obj.GetType()))
            {
                return false;
            }
            var id = obj as Id;
            return Name == id.Name;
        }

        public override int GetHashCode()
        {
            return base.GetHashCode();
        }

    }
}
