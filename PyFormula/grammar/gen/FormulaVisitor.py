# Generated from C:/Users/Qishen/Desktop/projects/FormulaCore/PyFormula/grammar\Formula.g4 by ANTLR 4.7.2
from antlr4 import *
if __name__ is not None and "." in __name__:
    from .FormulaParser import FormulaParser
else:
    from FormulaParser import FormulaParser

# This class defines a complete generic visitor for a parse tree produced by FormulaParser.

class FormulaVisitor(ParseTreeVisitor):

    # Visit a parse tree produced by FormulaParser#config.
    def visitConfig(self, ctx:FormulaParser.ConfigContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#sentenceConfig.
    def visitSentenceConfig(self, ctx:FormulaParser.SentenceConfigContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#settingList.
    def visitSettingList(self, ctx:FormulaParser.SettingListContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#setting.
    def visitSetting(self, ctx:FormulaParser.SettingContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#program.
    def visitProgram(self, ctx:FormulaParser.ProgramContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#importModule.
    def visitImportModule(self, ctx:FormulaParser.ImportModuleContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#moduleList.
    def visitModuleList(self, ctx:FormulaParser.ModuleListContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#module.
    def visitModule(self, ctx:FormulaParser.ModuleContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#modRefs.
    def visitModRefs(self, ctx:FormulaParser.ModRefsContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#modRef.
    def visitModRef(self, ctx:FormulaParser.ModRefContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#model.
    def visitModel(self, ctx:FormulaParser.ModelContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#modelBody.
    def visitModelBody(self, ctx:FormulaParser.ModelBodyContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#modelSentence.
    def visitModelSentence(self, ctx:FormulaParser.ModelSentenceContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#modelContractConf.
    def visitModelContractConf(self, ctx:FormulaParser.ModelContractConfContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#modelContract.
    def visitModelContract(self, ctx:FormulaParser.ModelContractContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#cardSpec.
    def visitCardSpec(self, ctx:FormulaParser.CardSpecContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#modelSigConfig.
    def visitModelSigConfig(self, ctx:FormulaParser.ModelSigConfigContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#modelIntro.
    def visitModelIntro(self, ctx:FormulaParser.ModelIntroContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#modelSig.
    def visitModelSig(self, ctx:FormulaParser.ModelSigContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#modelFactList.
    def visitModelFactList(self, ctx:FormulaParser.ModelFactListContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#modelFact.
    def visitModelFact(self, ctx:FormulaParser.ModelFactContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#domain.
    def visitDomain(self, ctx:FormulaParser.DomainContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#domainSigConfig.
    def visitDomainSigConfig(self, ctx:FormulaParser.DomainSigConfigContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#domainSig.
    def visitDomainSig(self, ctx:FormulaParser.DomainSigContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#domSentences.
    def visitDomSentences(self, ctx:FormulaParser.DomSentencesContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#domSentenceConfig.
    def visitDomSentenceConfig(self, ctx:FormulaParser.DomSentenceConfigContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#domSentence.
    def visitDomSentence(self, ctx:FormulaParser.DomSentenceContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#RegularTypeDecl.
    def visitRegularTypeDecl(self, ctx:FormulaParser.RegularTypeDeclContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#UnionTypeDecl.
    def visitUnionTypeDecl(self, ctx:FormulaParser.UnionTypeDeclContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#unnBody.
    def visitUnnBody(self, ctx:FormulaParser.UnnBodyContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#funcDecl.
    def visitFuncDecl(self, ctx:FormulaParser.FuncDeclContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#fields.
    def visitFields(self, ctx:FormulaParser.FieldsContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#field.
    def visitField(self, ctx:FormulaParser.FieldContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#unnElem.
    def visitUnnElem(self, ctx:FormulaParser.UnnElemContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#enumList.
    def visitEnumList(self, ctx:FormulaParser.EnumListContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#enumCnst.
    def visitEnumCnst(self, ctx:FormulaParser.EnumCnstContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#formulaRule.
    def visitFormulaRule(self, ctx:FormulaParser.FormulaRuleContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#setComprehension.
    def visitSetComprehension(self, ctx:FormulaParser.SetComprehensionContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#aggregation.
    def visitAggregation(self, ctx:FormulaParser.AggregationContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#oneArgAggregation.
    def visitOneArgAggregation(self, ctx:FormulaParser.OneArgAggregationContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#twoArgAggregation.
    def visitTwoArgAggregation(self, ctx:FormulaParser.TwoArgAggregationContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#threeArgAggregation.
    def visitThreeArgAggregation(self, ctx:FormulaParser.ThreeArgAggregationContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#disjunction.
    def visitDisjunction(self, ctx:FormulaParser.DisjunctionContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#conjunction.
    def visitConjunction(self, ctx:FormulaParser.ConjunctionContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#DerivedConstantConstraint.
    def visitDerivedConstantConstraint(self, ctx:FormulaParser.DerivedConstantConstraintContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#TermConstraint.
    def visitTermConstraint(self, ctx:FormulaParser.TermConstraintContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#SetEmptyConstraint.
    def visitSetEmptyConstraint(self, ctx:FormulaParser.SetEmptyConstraintContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#TypeConstraint.
    def visitTypeConstraint(self, ctx:FormulaParser.TypeConstraintContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#NamedTermConstraint.
    def visitNamedTermConstraint(self, ctx:FormulaParser.NamedTermConstraintContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#BinaryArithmeticConstraint.
    def visitBinaryArithmeticConstraint(self, ctx:FormulaParser.BinaryArithmeticConstraintContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#funcTerm.
    def visitFuncTerm(self, ctx:FormulaParser.FuncTermContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#funcTermList.
    def visitFuncTermList(self, ctx:FormulaParser.FuncTermListContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#BaseArithTerm.
    def visitBaseArithTerm(self, ctx:FormulaParser.BaseArithTermContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#AddSubArithTerm.
    def visitAddSubArithTerm(self, ctx:FormulaParser.AddSubArithTermContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#ParenWrappedArithTerm.
    def visitParenWrappedArithTerm(self, ctx:FormulaParser.ParenWrappedArithTermContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#ModArithTerm.
    def visitModArithTerm(self, ctx:FormulaParser.ModArithTermContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#MulDivArithTerm.
    def visitMulDivArithTerm(self, ctx:FormulaParser.MulDivArithTermContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#atom.
    def visitAtom(self, ctx:FormulaParser.AtomContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#constant.
    def visitConstant(self, ctx:FormulaParser.ConstantContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#binOp.
    def visitBinOp(self, ctx:FormulaParser.BinOpContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#relOp.
    def visitRelOp(self, ctx:FormulaParser.RelOpContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by FormulaParser#funModifier.
    def visitFunModifier(self, ctx:FormulaParser.FunModifierContext):
        return self.visitChildren(ctx)



del FormulaParser