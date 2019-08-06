# Generated from C:/Users/Qishen/Desktop/projects/FormulaCore/PyFormula/grammar\Formula.g4 by ANTLR 4.7.2
from antlr4 import *
if __name__ is not None and "." in __name__:
    from .FormulaParser import FormulaParser
else:
    from FormulaParser import FormulaParser

# This class defines a complete listener for a parse tree produced by FormulaParser.
class FormulaListener(ParseTreeListener):

    # Enter a parse tree produced by FormulaParser#config.
    def enterConfig(self, ctx:FormulaParser.ConfigContext):
        pass

    # Exit a parse tree produced by FormulaParser#config.
    def exitConfig(self, ctx:FormulaParser.ConfigContext):
        pass


    # Enter a parse tree produced by FormulaParser#sentenceConfig.
    def enterSentenceConfig(self, ctx:FormulaParser.SentenceConfigContext):
        pass

    # Exit a parse tree produced by FormulaParser#sentenceConfig.
    def exitSentenceConfig(self, ctx:FormulaParser.SentenceConfigContext):
        pass


    # Enter a parse tree produced by FormulaParser#settingList.
    def enterSettingList(self, ctx:FormulaParser.SettingListContext):
        pass

    # Exit a parse tree produced by FormulaParser#settingList.
    def exitSettingList(self, ctx:FormulaParser.SettingListContext):
        pass


    # Enter a parse tree produced by FormulaParser#setting.
    def enterSetting(self, ctx:FormulaParser.SettingContext):
        pass

    # Exit a parse tree produced by FormulaParser#setting.
    def exitSetting(self, ctx:FormulaParser.SettingContext):
        pass


    # Enter a parse tree produced by FormulaParser#program.
    def enterProgram(self, ctx:FormulaParser.ProgramContext):
        pass

    # Exit a parse tree produced by FormulaParser#program.
    def exitProgram(self, ctx:FormulaParser.ProgramContext):
        pass


    # Enter a parse tree produced by FormulaParser#importModule.
    def enterImportModule(self, ctx:FormulaParser.ImportModuleContext):
        pass

    # Exit a parse tree produced by FormulaParser#importModule.
    def exitImportModule(self, ctx:FormulaParser.ImportModuleContext):
        pass


    # Enter a parse tree produced by FormulaParser#moduleList.
    def enterModuleList(self, ctx:FormulaParser.ModuleListContext):
        pass

    # Exit a parse tree produced by FormulaParser#moduleList.
    def exitModuleList(self, ctx:FormulaParser.ModuleListContext):
        pass


    # Enter a parse tree produced by FormulaParser#module.
    def enterModule(self, ctx:FormulaParser.ModuleContext):
        pass

    # Exit a parse tree produced by FormulaParser#module.
    def exitModule(self, ctx:FormulaParser.ModuleContext):
        pass


    # Enter a parse tree produced by FormulaParser#modRefs.
    def enterModRefs(self, ctx:FormulaParser.ModRefsContext):
        pass

    # Exit a parse tree produced by FormulaParser#modRefs.
    def exitModRefs(self, ctx:FormulaParser.ModRefsContext):
        pass


    # Enter a parse tree produced by FormulaParser#modRef.
    def enterModRef(self, ctx:FormulaParser.ModRefContext):
        pass

    # Exit a parse tree produced by FormulaParser#modRef.
    def exitModRef(self, ctx:FormulaParser.ModRefContext):
        pass


    # Enter a parse tree produced by FormulaParser#model.
    def enterModel(self, ctx:FormulaParser.ModelContext):
        pass

    # Exit a parse tree produced by FormulaParser#model.
    def exitModel(self, ctx:FormulaParser.ModelContext):
        pass


    # Enter a parse tree produced by FormulaParser#modelBody.
    def enterModelBody(self, ctx:FormulaParser.ModelBodyContext):
        pass

    # Exit a parse tree produced by FormulaParser#modelBody.
    def exitModelBody(self, ctx:FormulaParser.ModelBodyContext):
        pass


    # Enter a parse tree produced by FormulaParser#modelSentence.
    def enterModelSentence(self, ctx:FormulaParser.ModelSentenceContext):
        pass

    # Exit a parse tree produced by FormulaParser#modelSentence.
    def exitModelSentence(self, ctx:FormulaParser.ModelSentenceContext):
        pass


    # Enter a parse tree produced by FormulaParser#modelContractConf.
    def enterModelContractConf(self, ctx:FormulaParser.ModelContractConfContext):
        pass

    # Exit a parse tree produced by FormulaParser#modelContractConf.
    def exitModelContractConf(self, ctx:FormulaParser.ModelContractConfContext):
        pass


    # Enter a parse tree produced by FormulaParser#modelContract.
    def enterModelContract(self, ctx:FormulaParser.ModelContractContext):
        pass

    # Exit a parse tree produced by FormulaParser#modelContract.
    def exitModelContract(self, ctx:FormulaParser.ModelContractContext):
        pass


    # Enter a parse tree produced by FormulaParser#cardSpec.
    def enterCardSpec(self, ctx:FormulaParser.CardSpecContext):
        pass

    # Exit a parse tree produced by FormulaParser#cardSpec.
    def exitCardSpec(self, ctx:FormulaParser.CardSpecContext):
        pass


    # Enter a parse tree produced by FormulaParser#modelSigConfig.
    def enterModelSigConfig(self, ctx:FormulaParser.ModelSigConfigContext):
        pass

    # Exit a parse tree produced by FormulaParser#modelSigConfig.
    def exitModelSigConfig(self, ctx:FormulaParser.ModelSigConfigContext):
        pass


    # Enter a parse tree produced by FormulaParser#modelIntro.
    def enterModelIntro(self, ctx:FormulaParser.ModelIntroContext):
        pass

    # Exit a parse tree produced by FormulaParser#modelIntro.
    def exitModelIntro(self, ctx:FormulaParser.ModelIntroContext):
        pass


    # Enter a parse tree produced by FormulaParser#modelSig.
    def enterModelSig(self, ctx:FormulaParser.ModelSigContext):
        pass

    # Exit a parse tree produced by FormulaParser#modelSig.
    def exitModelSig(self, ctx:FormulaParser.ModelSigContext):
        pass


    # Enter a parse tree produced by FormulaParser#modelFactList.
    def enterModelFactList(self, ctx:FormulaParser.ModelFactListContext):
        pass

    # Exit a parse tree produced by FormulaParser#modelFactList.
    def exitModelFactList(self, ctx:FormulaParser.ModelFactListContext):
        pass


    # Enter a parse tree produced by FormulaParser#modelFact.
    def enterModelFact(self, ctx:FormulaParser.ModelFactContext):
        pass

    # Exit a parse tree produced by FormulaParser#modelFact.
    def exitModelFact(self, ctx:FormulaParser.ModelFactContext):
        pass


    # Enter a parse tree produced by FormulaParser#domain.
    def enterDomain(self, ctx:FormulaParser.DomainContext):
        pass

    # Exit a parse tree produced by FormulaParser#domain.
    def exitDomain(self, ctx:FormulaParser.DomainContext):
        pass


    # Enter a parse tree produced by FormulaParser#domainSigConfig.
    def enterDomainSigConfig(self, ctx:FormulaParser.DomainSigConfigContext):
        pass

    # Exit a parse tree produced by FormulaParser#domainSigConfig.
    def exitDomainSigConfig(self, ctx:FormulaParser.DomainSigConfigContext):
        pass


    # Enter a parse tree produced by FormulaParser#domainSig.
    def enterDomainSig(self, ctx:FormulaParser.DomainSigContext):
        pass

    # Exit a parse tree produced by FormulaParser#domainSig.
    def exitDomainSig(self, ctx:FormulaParser.DomainSigContext):
        pass


    # Enter a parse tree produced by FormulaParser#domSentences.
    def enterDomSentences(self, ctx:FormulaParser.DomSentencesContext):
        pass

    # Exit a parse tree produced by FormulaParser#domSentences.
    def exitDomSentences(self, ctx:FormulaParser.DomSentencesContext):
        pass


    # Enter a parse tree produced by FormulaParser#domSentenceConfig.
    def enterDomSentenceConfig(self, ctx:FormulaParser.DomSentenceConfigContext):
        pass

    # Exit a parse tree produced by FormulaParser#domSentenceConfig.
    def exitDomSentenceConfig(self, ctx:FormulaParser.DomSentenceConfigContext):
        pass


    # Enter a parse tree produced by FormulaParser#domSentence.
    def enterDomSentence(self, ctx:FormulaParser.DomSentenceContext):
        pass

    # Exit a parse tree produced by FormulaParser#domSentence.
    def exitDomSentence(self, ctx:FormulaParser.DomSentenceContext):
        pass


    # Enter a parse tree produced by FormulaParser#RegularTypeDecl.
    def enterRegularTypeDecl(self, ctx:FormulaParser.RegularTypeDeclContext):
        pass

    # Exit a parse tree produced by FormulaParser#RegularTypeDecl.
    def exitRegularTypeDecl(self, ctx:FormulaParser.RegularTypeDeclContext):
        pass


    # Enter a parse tree produced by FormulaParser#UnionTypeDecl.
    def enterUnionTypeDecl(self, ctx:FormulaParser.UnionTypeDeclContext):
        pass

    # Exit a parse tree produced by FormulaParser#UnionTypeDecl.
    def exitUnionTypeDecl(self, ctx:FormulaParser.UnionTypeDeclContext):
        pass


    # Enter a parse tree produced by FormulaParser#unnBody.
    def enterUnnBody(self, ctx:FormulaParser.UnnBodyContext):
        pass

    # Exit a parse tree produced by FormulaParser#unnBody.
    def exitUnnBody(self, ctx:FormulaParser.UnnBodyContext):
        pass


    # Enter a parse tree produced by FormulaParser#funcDecl.
    def enterFuncDecl(self, ctx:FormulaParser.FuncDeclContext):
        pass

    # Exit a parse tree produced by FormulaParser#funcDecl.
    def exitFuncDecl(self, ctx:FormulaParser.FuncDeclContext):
        pass


    # Enter a parse tree produced by FormulaParser#fields.
    def enterFields(self, ctx:FormulaParser.FieldsContext):
        pass

    # Exit a parse tree produced by FormulaParser#fields.
    def exitFields(self, ctx:FormulaParser.FieldsContext):
        pass


    # Enter a parse tree produced by FormulaParser#field.
    def enterField(self, ctx:FormulaParser.FieldContext):
        pass

    # Exit a parse tree produced by FormulaParser#field.
    def exitField(self, ctx:FormulaParser.FieldContext):
        pass


    # Enter a parse tree produced by FormulaParser#unnElem.
    def enterUnnElem(self, ctx:FormulaParser.UnnElemContext):
        pass

    # Exit a parse tree produced by FormulaParser#unnElem.
    def exitUnnElem(self, ctx:FormulaParser.UnnElemContext):
        pass


    # Enter a parse tree produced by FormulaParser#enumList.
    def enterEnumList(self, ctx:FormulaParser.EnumListContext):
        pass

    # Exit a parse tree produced by FormulaParser#enumList.
    def exitEnumList(self, ctx:FormulaParser.EnumListContext):
        pass


    # Enter a parse tree produced by FormulaParser#enumCnst.
    def enterEnumCnst(self, ctx:FormulaParser.EnumCnstContext):
        pass

    # Exit a parse tree produced by FormulaParser#enumCnst.
    def exitEnumCnst(self, ctx:FormulaParser.EnumCnstContext):
        pass


    # Enter a parse tree produced by FormulaParser#formulaRule.
    def enterFormulaRule(self, ctx:FormulaParser.FormulaRuleContext):
        pass

    # Exit a parse tree produced by FormulaParser#formulaRule.
    def exitFormulaRule(self, ctx:FormulaParser.FormulaRuleContext):
        pass


    # Enter a parse tree produced by FormulaParser#setComprehension.
    def enterSetComprehension(self, ctx:FormulaParser.SetComprehensionContext):
        pass

    # Exit a parse tree produced by FormulaParser#setComprehension.
    def exitSetComprehension(self, ctx:FormulaParser.SetComprehensionContext):
        pass


    # Enter a parse tree produced by FormulaParser#disjunction.
    def enterDisjunction(self, ctx:FormulaParser.DisjunctionContext):
        pass

    # Exit a parse tree produced by FormulaParser#disjunction.
    def exitDisjunction(self, ctx:FormulaParser.DisjunctionContext):
        pass


    # Enter a parse tree produced by FormulaParser#conjunction.
    def enterConjunction(self, ctx:FormulaParser.ConjunctionContext):
        pass

    # Exit a parse tree produced by FormulaParser#conjunction.
    def exitConjunction(self, ctx:FormulaParser.ConjunctionContext):
        pass


    # Enter a parse tree produced by FormulaParser#TermConstraint.
    def enterTermConstraint(self, ctx:FormulaParser.TermConstraintContext):
        pass

    # Exit a parse tree produced by FormulaParser#TermConstraint.
    def exitTermConstraint(self, ctx:FormulaParser.TermConstraintContext):
        pass


    # Enter a parse tree produced by FormulaParser#TermConstraintWithAlias.
    def enterTermConstraintWithAlias(self, ctx:FormulaParser.TermConstraintWithAliasContext):
        pass

    # Exit a parse tree produced by FormulaParser#TermConstraintWithAlias.
    def exitTermConstraintWithAlias(self, ctx:FormulaParser.TermConstraintWithAliasContext):
        pass


    # Enter a parse tree produced by FormulaParser#BinaryArithmeticConstraint.
    def enterBinaryArithmeticConstraint(self, ctx:FormulaParser.BinaryArithmeticConstraintContext):
        pass

    # Exit a parse tree produced by FormulaParser#BinaryArithmeticConstraint.
    def exitBinaryArithmeticConstraint(self, ctx:FormulaParser.BinaryArithmeticConstraintContext):
        pass


    # Enter a parse tree produced by FormulaParser#AggregationCountConstraint.
    def enterAggregationCountConstraint(self, ctx:FormulaParser.AggregationCountConstraintContext):
        pass

    # Exit a parse tree produced by FormulaParser#AggregationCountConstraint.
    def exitAggregationCountConstraint(self, ctx:FormulaParser.AggregationCountConstraintContext):
        pass


    # Enter a parse tree produced by FormulaParser#SetComprehensionConstraint.
    def enterSetComprehensionConstraint(self, ctx:FormulaParser.SetComprehensionConstraintContext):
        pass

    # Exit a parse tree produced by FormulaParser#SetComprehensionConstraint.
    def exitSetComprehensionConstraint(self, ctx:FormulaParser.SetComprehensionConstraintContext):
        pass


    # Enter a parse tree produced by FormulaParser#DerivedConstantConstraint.
    def enterDerivedConstantConstraint(self, ctx:FormulaParser.DerivedConstantConstraintContext):
        pass

    # Exit a parse tree produced by FormulaParser#DerivedConstantConstraint.
    def exitDerivedConstantConstraint(self, ctx:FormulaParser.DerivedConstantConstraintContext):
        pass


    # Enter a parse tree produced by FormulaParser#TypeConstraint.
    def enterTypeConstraint(self, ctx:FormulaParser.TypeConstraintContext):
        pass

    # Exit a parse tree produced by FormulaParser#TypeConstraint.
    def exitTypeConstraint(self, ctx:FormulaParser.TypeConstraintContext):
        pass


    # Enter a parse tree produced by FormulaParser#funcTerm.
    def enterFuncTerm(self, ctx:FormulaParser.FuncTermContext):
        pass

    # Exit a parse tree produced by FormulaParser#funcTerm.
    def exitFuncTerm(self, ctx:FormulaParser.FuncTermContext):
        pass


    # Enter a parse tree produced by FormulaParser#funcTermList.
    def enterFuncTermList(self, ctx:FormulaParser.FuncTermListContext):
        pass

    # Exit a parse tree produced by FormulaParser#funcTermList.
    def exitFuncTermList(self, ctx:FormulaParser.FuncTermListContext):
        pass


    # Enter a parse tree produced by FormulaParser#ParenthesisArithTerm.
    def enterParenthesisArithTerm(self, ctx:FormulaParser.ParenthesisArithTermContext):
        pass

    # Exit a parse tree produced by FormulaParser#ParenthesisArithTerm.
    def exitParenthesisArithTerm(self, ctx:FormulaParser.ParenthesisArithTermContext):
        pass


    # Enter a parse tree produced by FormulaParser#AddSubArithTerm.
    def enterAddSubArithTerm(self, ctx:FormulaParser.AddSubArithTermContext):
        pass

    # Exit a parse tree produced by FormulaParser#AddSubArithTerm.
    def exitAddSubArithTerm(self, ctx:FormulaParser.AddSubArithTermContext):
        pass


    # Enter a parse tree produced by FormulaParser#ModArithTerm.
    def enterModArithTerm(self, ctx:FormulaParser.ModArithTermContext):
        pass

    # Exit a parse tree produced by FormulaParser#ModArithTerm.
    def exitModArithTerm(self, ctx:FormulaParser.ModArithTermContext):
        pass


    # Enter a parse tree produced by FormulaParser#MulDivArithTerm.
    def enterMulDivArithTerm(self, ctx:FormulaParser.MulDivArithTermContext):
        pass

    # Exit a parse tree produced by FormulaParser#MulDivArithTerm.
    def exitMulDivArithTerm(self, ctx:FormulaParser.MulDivArithTermContext):
        pass


    # Enter a parse tree produced by FormulaParser#AtomTerm.
    def enterAtomTerm(self, ctx:FormulaParser.AtomTermContext):
        pass

    # Exit a parse tree produced by FormulaParser#AtomTerm.
    def exitAtomTerm(self, ctx:FormulaParser.AtomTermContext):
        pass


    # Enter a parse tree produced by FormulaParser#atom.
    def enterAtom(self, ctx:FormulaParser.AtomContext):
        pass

    # Exit a parse tree produced by FormulaParser#atom.
    def exitAtom(self, ctx:FormulaParser.AtomContext):
        pass


    # Enter a parse tree produced by FormulaParser#constant.
    def enterConstant(self, ctx:FormulaParser.ConstantContext):
        pass

    # Exit a parse tree produced by FormulaParser#constant.
    def exitConstant(self, ctx:FormulaParser.ConstantContext):
        pass


    # Enter a parse tree produced by FormulaParser#binOp.
    def enterBinOp(self, ctx:FormulaParser.BinOpContext):
        pass

    # Exit a parse tree produced by FormulaParser#binOp.
    def exitBinOp(self, ctx:FormulaParser.BinOpContext):
        pass


    # Enter a parse tree produced by FormulaParser#relOp.
    def enterRelOp(self, ctx:FormulaParser.RelOpContext):
        pass

    # Exit a parse tree produced by FormulaParser#relOp.
    def exitRelOp(self, ctx:FormulaParser.RelOpContext):
        pass


    # Enter a parse tree produced by FormulaParser#funModifier.
    def enterFunModifier(self, ctx:FormulaParser.FunModifierContext):
        pass

    # Exit a parse tree produced by FormulaParser#funModifier.
    def exitFunModifier(self, ctx:FormulaParser.FunModifierContext):
        pass


