import logging
import datetime
import networkx as nx
from antlr4 import *
from collections import Counter

from grammar.nodes.term import *
from grammar.nodes.constraint import *
from grammar.visitor import ExprVisitor
from grammar.gen.FormulaLexer import FormulaLexer
from grammar.gen.FormulaParser import FormulaParser

from executer.modules.domain import Domain
from executer.modules.model import Model
from executer.term import *
from executer.relation import *
from executer.constraint import *
from executer.rule import *


class Compiler:
    # e.g. fact_map = {link: [[a,b], [b,c]]}
    def __init__(self, logger_disabled=False):
        self.programs = {}
        self.logger = logging.getLogger(__name__)
        if not self.logger.handlers:
            self.logger.addHandler(logging.StreamHandler())
        self.logger.setLevel(logging.DEBUG)
        if logger_disabled:
            self.logger.disabled = True

    def print_all_facts(self, model_name):
        model = self.find_model_by_name(model_name)
        for index in model.type_index_map.values():
            self.logger.debug(index)

    def execute_model(self, model_name):
        model = self.find_model_by_name(model_name)
        if model:
            model.compile()
        else:
            self.logger.info('Cannot find the model with name %s' % model_name)

    def generate_model(self, domain_name, model_name):
        domain = self.find_domain_by_name(domain_name)
        model = Model(model_name, domain)
        domain.model_map[model] = model
        return model

    def make_changes_and_execute(self, model_name, changes):
        model = self.find_model_by_name(model_name)
        if model:
            model.add_changes(changes)
        else:
            self.logger.info('Cannot find the model with name %s' % model_name)

    def find_domain_by_name(self, name):
        for program in self.programs:
            domain_map = self.programs[program]
            for domain in domain_map.values():
                if domain.domain_name == name:
                    return domain
        raise Exception('No domain with name %s is found.' % name)

    def find_model_by_name(self, name):
        for program in self.programs:
            domain_map = self.programs[program]
            for domain in domain_map.values():
                for model in domain.model_map.values():
                    if model.model_name == name:
                        return model
        raise Exception('No model with name %s is found.' % name)

    def parse(self, file_str=None, filename=None):
        if file_str:
            filename = 'Program-' + str(hash(file_str)) + '.4ml'
            stream = InputStream(file_str)
        else:
            stream = FileStream(filename)
        lexer = FormulaLexer(stream)
        stream = CommonTokenStream(lexer)
        parser = FormulaParser(stream)
        program = parser.program()
        domain_map, model_map = self.load_program(program)
        self.programs[filename] = domain_map

    def load_program(self, program):
        visitor = ExprVisitor()
        visitor.visit(program)
        domain_map = {}
        model_map = {}

        for name in visitor.domains:
            domain_node = visitor.domains[name]
            domain_name = domain_node.domain_sig.name

            # Store all type and rule definitions.
            type_map = {}
            rules = []
            conforms = []

            # Add built-in types
            type_map.update(BuiltInType.get_types())

            # Types should be sorted after validation so Edge does not occur before Node.
            for type_node in domain_node.types:
                basic_type = BasicType(type_node.name, type_node.labels, type_node.types)
                type_map[type_node.name] = basic_type

            # Add all rules to a domain
            for rule_node in domain_node.rules:
                head_preds = []
                for term_node in rule_node.head:
                    term = self.load_term_node(term_node, type_map)
                    pred = Predicate(term)
                    head_preds.append(pred)

                body_constraints = []
                for conjunction in rule_node.body:
                    for constraint_node in conjunction:
                        if type(constraint_node) is TermConstraintNode:
                            has_negation = constraint_node.has_negation
                            term_node = constraint_node.term
                            term = self.load_term_node(term_node, type_map)
                            alias = constraint_node.alias
                            constraint = Predicate(term, negated=has_negation)
                            body_constraints.append(constraint)
                        else:
                            # TODO: Add other constraints
                            pass
                rule = Rule(head_preds, body_constraints)
                rules.append(rule)

            conforms = []
            domain = Domain(domain_name, type_map, rules, conforms, logger=self.logger)
            domain_map[domain_name] = domain

        # Model facts should be sorted too, so alias occurs after its definition.
        for model_name in visitor.models:
            facts = []
            alias_to_fact_map = {}
            model_node = visitor.models[model_name]
            # ModelFactListNode
            model_fact_list_node = model_node.fact_list_node
            alias_map = model_fact_list_node.alias_map
            fact_nodes = model_fact_list_node.facts
            # ModelSigConfigNode
            model_sig_node = model_node.domain_sig
            model_name = model_sig_node.model_name
            domain_name = model_sig_node.domain

            for fact_node in fact_nodes:
                if fact_node in alias_map:
                    alias = alias_map[fact_node]
                else:
                    alias = None

                if type(fact_node) is CompositeTermNode:
                    fact = self.load_term_node(fact_node, type_map)
                    if alias:
                        fact.alias = alias
                        alias_to_fact_map[alias] = fact
                    facts.append(fact)

            # Replace all model reference with model facts.
            for fact in facts:
                self.replace_variables(fact, alias_to_fact_map)

            domain = domain_map[domain_name]
            model = Model(model_name, domain, facts)
            domain.add_model(model_name, model)
            model_map[model_name] = model

        return domain_map, model_map

    def replace_variables(self, term, alias_to_fact_map):
        """
        Replace variables in the term based on the alias to fact mapping.
        :param term:
        :param alias_to_fact_map:
        :return:
        """
        if type(term) is Composite:
            for i, subterm in enumerate(term.args):
                if type(subterm) is Variable:
                    var_name = subterm.var
                    if var_name in alias_to_fact_map:
                        term.args[i] = alias_to_fact_map[var_name]
                elif type(subterm) is Composite:
                    self.replace_variables(subterm, alias_to_fact_map)

    def load_term_node(self, term_node, type_map, var_type=None):
        # Turn AST node into a real FORMULA term.
        if type(term_node) is CompositeTermNode:
            type_name = term_node.type
            sort = type_map[type_name]
            terms = []
            for i, subterm_node in enumerate(term_node.terms):
                # Need to find the type for variable term
                subterm_type_str = sort.types[i]
                subterm_type = type_map[subterm_type_str]
                subterm = self.load_term_node(subterm_node, type_map, subterm_type)
                terms.append(subterm)
            term = Composite(sort, terms)
            return term
        elif type(term_node) is VariableTermNode:
            return Variable(term_node.variable, var_type)
        elif type(term_node) is ConstantNode:
            # TODO: Add type to atom node as well.
            return Atom(term_node.constant)
