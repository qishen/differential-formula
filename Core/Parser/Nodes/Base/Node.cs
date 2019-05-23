using System;
using System.Collections.Generic;
using System.Diagnostics.Contracts;
using System.Linq;
using System.Text;
using System.Threading;

using Antlr4.Runtime;

namespace Microsoft.Formula.Core.Parser.Nodes
{
    public abstract class Node : INode
    {
        public ParserRuleContext SourceLocation { get; }

        public bool IsParamType
        {
            get { return IsTypeTerm || NodeKind == NodeKind.ModRef; }
        }

        public bool IsTypeTerm
        {
            get { return NodeKind == NodeKind.UnnDecl || IsUnionComponent; }
        }

        public bool IsUnionComponent
        {
            get { return NodeKind == NodeKind.Id || NodeKind == NodeKind.Enum; }
        }

        public bool IsEnumElement
        {
            get { return NodeKind == NodeKind.Id || NodeKind == NodeKind.Cnst || NodeKind == NodeKind.Range; }
        }

        public bool IsAtom
        {
            get { return NodeKind == NodeKind.Id || NodeKind == NodeKind.Cnst; }
        }

        public bool IsFuncOrAtom
        {
            get { return NodeKind == NodeKind.Id || NodeKind == NodeKind.Cnst || NodeKind == NodeKind.FuncTerm || NodeKind == NodeKind.Compr || NodeKind == NodeKind.Quote; }
        }

        public bool IsModAppArg
        {
            get { return NodeKind == NodeKind.Id || NodeKind == NodeKind.Cnst || NodeKind == NodeKind.FuncTerm || NodeKind == NodeKind.ModRef || NodeKind == NodeKind.Quote; }
        }

        public bool IsDomOrTrans
        {
            get { return NodeKind == NodeKind.Domain || NodeKind == NodeKind.Transform; }
        }

        public bool IsModule
        {
            get { return NodeKind == NodeKind.Domain || NodeKind == NodeKind.Transform || NodeKind == NodeKind.TSystem || NodeKind == NodeKind.Model || NodeKind == NodeKind.Machine; }
        }

        public bool IsTypeDecl
        {
            get { return NodeKind == NodeKind.ConDecl || NodeKind == NodeKind.MapDecl || NodeKind == NodeKind.UnnDecl; }
        }

        public bool IsConstraint
        {
            get { return NodeKind == NodeKind.Find || NodeKind == NodeKind.RelConstr; }
        }

        public abstract NodeKind NodeKind
        {
            get;
        }

        internal Node(ParserRuleContext sourceLocation)
        {
            SourceLocation = sourceLocation;
        }

        internal Node()
        { }

        public void Visit()
        {

        }
        
    }
}

