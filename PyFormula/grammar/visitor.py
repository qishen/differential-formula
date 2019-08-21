from grammar.gen.FormulaParser import FormulaParser
from grammar.gen.FormulaVisitor import FormulaVisitor

from grammar.nodes.module import ModuleRefNode
from grammar.nodes.domain import DomainNode, DomainSigConfigNode, InheritanceType
from grammar.nodes.model import ModelFactListNode, ModelNode, ModelSigConfigNode
from grammar.nodes.type import BasicTypeNode, UnionTypeNode, TypeRefNode
from grammar.nodes.enum import EnumNode, EnumRangeCnstNode, EnumCnstNode
from grammar.nodes.term import CompositeTermNode, VariableTermNode, ConstantNode
from grammar.nodes.rule import RuleNode
from grammar.nodes.constraint import TermConstraintNode, BinaryConstraintNode, TypeConstraintNode
from grammar.nodes.aggregation import SetComprehensionNode, AggregationNode
from grammar.nodes.expression import BinOp, RelOp, ArithmeticExprNode

from executer.rule import Rule
from executer.relation import *


class ExprVisitor(FormulaVisitor):
    def __init__(self):
        self.domains = {}
        self.models = {}

    def visitModRefs(self, ctx:FormulaParser.ModRefsContext):
        modelrefs = []
        for modelref in ctx.modRef():
            node = self.visit(modelref)
            modelrefs.append(node)
        return modelrefs

    def visitModRef(self, ctx:FormulaParser.ModRefContext):
        path = None
        rename = None
        if ctx.STRING():
            # Module not defined in current program and path refers to the location that has defined the module.
            path = ctx.STRING().getText().strip('\"')
        if ctx.RENAMES():
            rename = ctx.BId(0).getText()
            module_name = ctx.BId(1).getText()
        else:
            module_name = ctx.BId(0).getText()
        return ModuleRefNode(module_name, rename, path)

    def visitDomain(self, ctx:FormulaParser.DomainContext):
        sig_node = self.visit(ctx.domainSigConfig())
        sentence_nodes = self.visit(ctx.domSentences())
        domain_node = DomainNode(sig_node, sentence_nodes)
        domain_name = sig_node.name
        self.domains[domain_name] = domain_node
        return domain_node

    def visitDomainSigConfig(self, ctx:FormulaParser.DomainSigConfigContext):
        # TODO: figure out what config is used for
        # ctx.config()
        domain_name, inherit_type, modrefs = self.visit(ctx.domainSig())
        return DomainSigConfigNode(domain_name, inherit_type, modrefs)

    def visitDomainSig(self, ctx:FormulaParser.DomainSigContext):
        domain_name = ctx.BId().getText()
        mod_ref_nodes = None
        inherit_type = None
        if ctx.modRefs():
            mod_ref_nodes = self.visit(ctx.modRefs())
            if ctx.INCLUDES():
                inherit_type = InheritanceType.INCLUDES
            elif ctx.EXTENDS():
                inherit_type = InheritanceType.EXTENDS
            else:
                inherit_type = InheritanceType.NONE
        # The last two can be None.
        return domain_name, inherit_type, mod_ref_nodes

    def visitDomSentences(self, ctx:FormulaParser.DomSentencesContext):
        # A list of type node, rule node or conformance node.
        nodes = []
        for domSentenceConfig in ctx.domSentenceConfig():
            node = self.visit(domSentenceConfig)
            nodes.append(node)
        return nodes

    def visitDomSentenceConfig(self, ctx:FormulaParser.DomSentenceConfigContext):
        # TODO: figure out how to use sentenceConfig
        node = self.visit(ctx.domSentence())
        return node

    def visitDomSentence(self, ctx:FormulaParser.DomSentenceContext):
        node = None
        if ctx.typeDecl():
            node = self.visit(ctx.typeDecl())
        elif ctx.formulaRule():
            node = self.visit(ctx.formulaRule())
        elif ctx.CONFORMS():
            node = self.visit(ctx.disjunction())
        return node

    def visitRegularTypeDecl(self, ctx:FormulaParser.RegularTypeDeclContext):
        type_name = ctx.BId().getText()
        labels = []
        types = []
        field_tuples = self.visit(ctx.fields())
        for field_tuple in field_tuples:
            (label, has_any, type_or_union) = field_tuple
            if label:
                labels.append(label)
            else:
                labels.append(None)
            # types contains either union node or a list of strings.
            types.append(type_or_union)
        return BasicTypeNode(type_name, labels, types)

    def visitFields(self, ctx:FormulaParser.FieldsContext):
        fields = []
        for field in ctx.field():
            label, has_any, type_or_union = self.visit(field)
            fields.append((label, has_any, type_or_union))
        return fields

    def visitField(self, ctx:FormulaParser.FieldContext):
        label = None
        if ctx.BId():
            label = ctx.BId().getText()
        has_any = False
        if ctx.ANY():
            has_any = True
        if ctx.unnBody():
            # union is just a list of strings
            union = self.visit(ctx.unnBody())
            return label, has_any, union
        elif ctx.qualId():
            # type_name is a list of strings
            type_name = self.visit(ctx.qualId())
            if len(type_name) > 1:
                type_ref_node = TypeRefNode(type_name[1], ref_name=type_name[0])
            else:
                type_ref_node = TypeRefNode(type_name[0])
            return label, has_any, type_ref_node

    def visitUnionTypeDecl(self, ctx:FormulaParser.UnionTypeDeclContext):
        type_name = ctx.BId().getText()
        subtypes = self.visit(ctx.unnBody())
        return UnionTypeNode(type_name, subtypes)

    def visitUnnBody(self, ctx:FormulaParser.UnnBodyContext):
        # return a list of id or EnumNode.
        subtypes = []
        for unnElem in ctx.unnElem():
            node = self.visit(unnElem)
            subtypes.append(node)
        return subtypes

    def visitUnnElem(self, ctx:FormulaParser.UnnElemContext):
        if ctx.BId():
            return ctx.BId().getText()
        else:
            return self.visit(ctx.enumList())

    def visitEnumList(self, ctx:FormulaParser.EnumListContext):
        # A list of constants or range nodes.
        enum_list = []
        for enum_cnst in ctx.enumCnst():
            node = self.visit(enum_cnst)
            enum_list.append(node)
        return EnumNode(enum_list)

    def visitEnumCnst(self, ctx:FormulaParser.EnumCnstContext):
        if ctx.RANGE():
            low_str = ctx.DECIMAL(0).getText()
            high_str = ctx.DECIMAL(1).getText()
            return EnumRangeCnstNode(low_str, high_str)
        elif ctx.BId():
            user_defined_constant = ctx.BId().getText()
            return EnumCnstNode(ConstantNode(user_defined_constant))
        elif ctx.constant():
            constant_node = self.visit(ctx.constant())
            return EnumCnstNode(constant_node)

    def visitQualId(self, ctx:FormulaParser.QualIdContext):
        bids = []
        for bid in ctx.BId():
            bids.append(bid.getText())
        return bids

    def visitConstant(self, ctx:FormulaParser.ConstantContext):
        constant = None
        if ctx.DECIMAL():
            constant = int(ctx.DECIMAL().getText())
        elif ctx.REAL():
            constant = float(ctx.REAL().getText())
        elif ctx.FRAC():
            pass
        elif ctx.STRING():
            constant = ctx.STRING().getText().strip('\"')
        else:
            raise Exception('Wrong input to represent constants!')
        return ConstantNode(constant)

    def visitModel(self, ctx:FormulaParser.ModelContext):
        if ctx.modelBody():
            facts_node, partial_sentences = self.visit(ctx.modelBody())
        sig_node = self.visit(ctx.modelSigConfig())
        model_name = sig_node.model_name
        model_node = ModelNode(sig_node, facts_node)
        self.models[model_name] = model_node
        return model_node

    def visitModelSigConfig(self, ctx:FormulaParser.ModelSigConfigContext):
        # TODO: figure out what config is used for.
        # ctx.config()
        msc = self.visit(ctx.modelSig())
        return msc

    def visitModelSig(self, ctx:FormulaParser.ModelSigContext):
        is_partial, model_name, model_ref_name = self.visit(ctx.modelIntro())
        # some model refs to be included or extended
        # model_refs = self.visit(ctx.modRefs())
        return ModelSigConfigNode(is_partial, model_name, model_ref_name)

    def visitModelIntro(self, ctx:FormulaParser.ModelIntroContext):
        is_partial = False
        if ctx.PARTIAL():
            is_partial = True
        model_name = ctx.BId().getText()
        model_ref_name = self.visit(ctx.modRef())
        return is_partial, model_name, model_ref_name

    def visitModelBody(self, ctx:FormulaParser.ModelBodyContext):
        fact_sentence_nodes = []
        partial_model_sentence_nodes = []
        for model_sentence in ctx.modelSentence():
            sentence = self.visit(model_sentence)
            if type(sentence) is ModelFactListNode:
                fact_sentence_nodes.append(sentence)
            else:
                partial_model_sentence_nodes.append(sentence)

        # Combine all model facts in each sentence together.
        alias_map = {}
        facts = []
        for model_fact_list_node in fact_sentence_nodes:
            alias_map = {**alias_map, **model_fact_list_node.alias_map}
            facts = facts + model_fact_list_node.facts
        new_model_fact_list_node = ModelFactListNode(alias_map, facts)

        # TODO: Add partial model sentences
        partial_sentences = None
        return new_model_fact_list_node, partial_sentences

    def visitModelSentence(self, ctx:FormulaParser.ModelSentenceContext):
        if ctx.modelFactList():
            alias_map, facts = self.visit(ctx.modelFactList())
            return ModelFactListNode(alias_map, facts)
        elif ctx.modelContractConf():
            # TODO: this part is for implementing model sentence in partial model.
            pass

    def visitModelContractConf(self, ctx:FormulaParser.ModelContractConfContext):
        # TODO: parse sentences in a partial model.
        pass

    def visitModelFact(self, ctx:FormulaParser.ModelFactContext):
        alias = None
        if ctx.BId():
            alias = ctx.BId().getText()
        fact = self.visit(ctx.funcTerm())
        return alias, fact

    def visitModelFactList(self, ctx:FormulaParser.ModelFactListContext):
        facts = []
        alias_map = {}
        for fact in ctx.modelFact():
            alias, fact_node = self.visit(fact)
            facts.append(fact_node)
            if alias:
                alias_map[fact_node] = alias
        return alias_map, facts

    def visitFormulaRule(self, ctx:FormulaParser.FormulaRuleContext):
        head = self.visit(ctx.funcTermList())
        body = self.visit(ctx.disjunction())
        return RuleNode(head, body)

    def visitFuncTermList(self, ctx:FormulaParser.FuncTermListContext):
        terms = []
        for functerm in ctx.funcTerm():
            term = self.visit(functerm)
            terms.append(term)
        return terms

    def visitFuncTerm(self, ctx:FormulaParser.FuncTermContext):
        if ctx.atom():
            return self.visit(ctx.atom())
        else:
            # type_name could be a list of string, e.g. in.Edge(x,y).
            type_name = self.visit(ctx.qualId())
            type_ref_node = None
            if len(type_name) > 1:
                type_ref_node = TypeRefNode(type_name[1], ref_name=type_name[0])
            else:
                type_ref_node = TypeRefNode(type_name[0])
            terms = []
            for functerm in ctx.funcTerm():
                functerm = self.visit(functerm)
                terms.append(functerm)
            return CompositeTermNode(type_ref_node, terms)

    def visitAtom(self, ctx:FormulaParser.AtomContext):
        if ctx.qualId():
            # variable could be a list of strings.
            variable = self.visit(ctx.qualId())
            return VariableTermNode(variable)
        else:
            return self.visit(ctx.constant())

    def visitDisjunction(self, ctx:FormulaParser.DisjunctionContext):
        conjunctions = []
        for conjunction in ctx.conjunction():
            conjunction_node = self.visit(conjunction)
            conjunctions.append(conjunction_node)
        return conjunctions

    def visitConjunction(self, ctx:FormulaParser.ConjunctionContext):
        constraints = []
        for constraint in ctx.constraint():
            constraint_node = self.visit(constraint)
            constraints.append(constraint_node)
        return constraints

    def visitSetComprehension(self, ctx:FormulaParser.SetComprehensionContext):
        terms = self.visit(ctx.funcTermList())
        constraints = self.visit(ctx.conjunction())
        return SetComprehensionNode(terms, constraints)

    def visitTermConstraint(self, ctx:FormulaParser.TermConstraintContext):
        has_negation = False
        if ctx.NO():
            has_negation = True
        functerm = self.visit(ctx.funcTerm())
        return TermConstraintNode(has_negation, functerm)

    def visitNamedTermConstraint(self, ctx:FormulaParser.NamedTermConstraintContext):
        alias = self.visit(ctx.qualId())
        functerm = self.visit(ctx.funcTerm())
        # Named term constraint cannot be negated.
        return TermConstraintNode(False, functerm, alias)

    def visitSetEmptyConstraint(self, ctx:FormulaParser.SetEmptyConstraintContext):
        # Return a binary constraint node with the count of set comprehension equal to zero.
        setcompr_node = self.visit(ctx.setComprehension())
        aggregation_node = AggregationNode('count', setcompr_node)
        zero = ConstantNode(0)
        return BinaryConstraintNode(aggregation_node, zero, RelOp.EQ)

    def visitRelOp(self, ctx:FormulaParser.RelOpContext):
        if ctx.EQ():
            return RelOp.EQ
        elif ctx.NE():
            return RelOp.NEQ
        elif ctx.LT():
            return RelOp.LT
        elif ctx.LE():
            return RelOp.LE
        elif ctx.GT():
            return RelOp.GT
        elif ctx.GE():
            return RelOp.GE

    def visitBinaryArithmeticConstraint(self, ctx:FormulaParser.BinaryArithmeticConstraintContext):
        op = self.visit(ctx.relOp())
        left = self.visit(ctx.arithmeticTerm(0))
        right = self.visit(ctx.arithmeticTerm(1))
        return BinaryConstraintNode(left, right, op)

    def visitDerivedConstantConstraint(self, ctx:FormulaParser.DerivedConstantConstraintContext):
        # variable must be of boolean type.
        pass

    def visitTypeConstraint(self, ctx:FormulaParser.TypeConstraintContext):
        variable = self.visit(ctx.qualId(0))
        type_name = self.visit(ctx.qualId(1))
        return TypeConstraintNode(variable, type_name)

    def visitParenWrappedArithTerm(self, ctx:FormulaParser.ParenWrappedArithTermContext):
        return self.visit(ctx.arithmeticTerm())

    def visitMulDivArithTerm(self, ctx:FormulaParser.MulDivArithTermContext):
        op = None
        if ctx.MUL():
            op = BinOp.MUL
        elif ctx.DIV():
            op = BinOp.DIV
        left = self.visit(ctx.arithmeticTerm(0))
        right = self.visit(ctx.arithmeticTerm(1))
        return ArithmeticExprNode(left, right, op)

    def visitModArithTerm(self, ctx:FormulaParser.ModArithTermContext):
        op = BinOp.MOD
        return ArithmeticExprNode(ctx.arithmeticTerm(0), ctx.arithmeticTerm(1), op)

    def visitAddSubArithTerm(self, ctx:FormulaParser.AddSubArithTermContext):
        op = None
        if ctx.PLUS():
            op = BinOp.PLUS
        elif ctx.MINUS():
            op = BinOp.MINUS
        left = self.visit(ctx.arithmeticTerm(0))
        right = self.visit(ctx.arithmeticTerm(1))
        return ArithmeticExprNode(left, right, op)

    def visitBaseArithTerm(self, ctx:FormulaParser.BaseArithTermContext):
        if ctx.atom():
            return self.visit(ctx.atom())
        elif ctx.aggregation():
            aggregation = self.visit(ctx.aggregation())
            return aggregation

    def visitOneArgAggregation(self, ctx:FormulaParser.OneArgAggregationContext):
        func = ctx.BId().getText()
        setcompr_node = self.visit(ctx.setComprehension())
        return AggregationNode(func, setcompr_node)

    def visitTwoArgAggregation(self, ctx:FormulaParser.TwoArgAggregationContext):
        func = ctx.BId().getText()
        default = self.visit(ctx.constant())
        setcompr_node = self.visit(ctx.setComprehension())
        return AggregationNode(func, setcompr_node, default_value=default)

    def visitThreeArgAggregation(self, ctx:FormulaParser.ThreeArgAggregationContext):
        func = ctx.BId().getText()
        tid = ctx.TID().getText()
        term = self.visit(ctx.funcTerm())
        setcompr_node = self.visit(ctx.setComprehension())
        return AggregationNode(func, setcompr_node, tid=tid, default_value=term)






































