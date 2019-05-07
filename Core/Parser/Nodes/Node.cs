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

        private static readonly Node[] noNodes = new Node[0];

        public bool IsQuoteItem
        {
            get { return IsFuncOrAtom || NodeKind == NodeKind.QuoteRun; }
        }

        public bool IsContractSpec
        {
            get { return NodeKind == NodeKind.Body || NodeKind == NodeKind.CardPair; }
        }

        public bool IsParamType
        {
            get { return IsTypeTerm || NodeKind == NodeKind.ModRef; }
        }

        public bool IsTypeTerm
        {
            get { return NodeKind == NodeKind.Union || IsUnionComponent; }
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

        public bool IsConfigSettable
        {
            get
            {
                return NodeKind == NodeKind.Rule ||
                       NodeKind == NodeKind.Step ||
                       NodeKind == NodeKind.Update ||
                       NodeKind == NodeKind.Property ||
                       NodeKind == NodeKind.ContractItem ||
                       NodeKind == NodeKind.ModelFact ||
                       IsTypeDecl;
            }
        }

        [Pure]
        public bool CanHaveContract(ContractKind kind)
        {
            switch (kind)
            {
                case ContractKind.ConformsProp:
                    return NodeKind == NodeKind.Domain;
                case ContractKind.EnsuresProp:
                case ContractKind.RequiresProp:
                    return NodeKind == NodeKind.Transform || NodeKind == NodeKind.Model;
                case ContractKind.RequiresSome:
                case ContractKind.RequiresAtLeast:
                case ContractKind.RequiresAtMost:
                    return NodeKind == NodeKind.Model;
                default:
                    throw new NotImplementedException();
            }
        }

        public abstract NodeKind NodeKind
        {
            get;
        }

        internal object CompilerData
        {
            get;
            set;
        }

        internal Node(ParserRuleContext sourceLocation)
        {
            SourceLocation = sourceLocation;
        }

        
    }
}

