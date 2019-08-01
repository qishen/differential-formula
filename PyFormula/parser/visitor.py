from parser.gen.FormulaVisitor import FormulaVisitor
from parser.gen.FormulaParser import FormulaParser

from modules.term import Term, Atom, Variable, Composite
from modules.relation import Relation


class ExprVisitor(FormulaVisitor):
    def __init__(self):
        self.relations = []
        self.rules = []

    def visitDomain(self, ctx:FormulaParser.DomainContext):
        domain_sig_ctx = ctx.domainSig()
        domain_name = domain_sig_ctx.Id().getText()
        sentences = self.visit(ctx.domSentences())

    def visitModel(self, ctx:FormulaParser.ModelContext):
        pass
