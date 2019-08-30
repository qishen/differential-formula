import logging
import datetime
import networkx as nx
from antlr4 import *
from collections import Counter

from grammar.visitor import ExprVisitor
from grammar.gen.FormulaLexer import FormulaLexer
from grammar.gen.FormulaParser import FormulaParser

from grammar.nodes.enum import *
from grammar.nodes.domain import *
from grammar.nodes.term import *
from grammar.nodes.constraint import *
from grammar.nodes.type import *
from grammar.nodes.aggregation import *
from grammar.nodes.expression import *

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

    def clear_all(self):
        self.programs.clear()

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
        domain_map, model_map = self.load_program(program, filename)
        self.programs[filename] = domain_map

    def load_program(self, program, filename):
        visitor = ExprVisitor()
        visitor.visit(program)
        domain_map = {}
        model_map = {}

        for name in visitor.domains:
            domain_node = visitor.domains[name]
            domain_sig = domain_node.domain_sig
            domain_name = domain_sig.name
            modref_nodes = domain_sig.modrefs
            inherit_type = domain_sig.inherit_type

            # Suppose inherited domains are always loaded first, then map renamed id to domain.
            inherited_domain_map = {}
            if modref_nodes:
                for modref_node in modref_nodes:
                    # Need to find the domain map in another program.
                    if modref_node.path is not None:
                        if modref_node.path not in self.programs:
                            raise Exception('Wrong path and cannot find the program with path %s' % modref_node.path)
                        else:
                            if modref_node.module in self.programs[modref_node.path]:
                                domain = self.programs[modref_node.path][modref_node.module]
                            else:
                                raise Exception('Domain %s is not defined in program %s' % modref_node.module, modref_node.path)
                    else:
                        if modref_node.module in domain_map:
                            domain = domain_map[modref_node.module]
                        else:
                            raise Exception('Domain %s is not defined in current program' % modref_node.module)

                    if modref_node.rename:
                        # Domain definition with rename, e.g. domain IsoDAGs extends Left::DAGs, Right::DAGs
                        inherited_domain_map[modref_node.rename] = domain
                    else:
                        # Domain definition without rename, then use domain name as the key in map
                        inherited_domain_map[modref_node.module] = domain

            # Store all type and rule definitions.
            type_map = {}
            rules = []
            conforms = []

            # Add built-in types
            built_in_type_map = BuiltInType.get_types()
            for type_name in built_in_type_map:
                type_map[type_name] = built_in_type_map[type_name]

            # Types should be sorted after validation so Edge does not occur before Node.
            for type_node in domain_node.types:
                if type(type_node) is BasicTypeNode:
                    types = []
                    refs = []
                    labels = type_node.labels
                    subtype_nodes = type_node.types
                    # subtype node can be either regular type or an union of types.
                    for subtype_node in subtype_nodes:
                        if type(subtype_node) is list:
                            # subtype_node can be a list of string or EnumNode e.g. Node + Edge + {NIL}
                            for component in subtype_node:
                                if type(component) is str:
                                    types.append(component)
                                    refs.append(None)
                                elif type(component) is EnumNode:
                                    # Create a new enum type to represent a list of constants.
                                    enum_items = []
                                    for cnst_node in component.items:
                                        if type(cnst_node) is EnumCnstNode:
                                            enum_items.append(cnst_node.constant.constant)
                                        elif type(cnst_node) is EnumRangeCnstNode:
                                            # TODO:
                                            pass
                                    assigned_enum_name = 'enum_' + str(hash(tuple(enum_items)))
                                    enum_type = EnumType(assigned_enum_name, enum_items)
                                    type_map[assigned_enum_name] = enum_type
                                    types.append(assigned_enum_name)
                                    refs.append(None)
                        elif type(subtype_node) is TypeRefNode:
                            # Need to consider domain alias like Left::DAGs and Right::DAGs
                            types.append(subtype_node.type)
                            refs.append(subtype_node.ref)

                    basic_type = BasicType(type_node.name, labels=labels, types=types, refs=refs)
                    type_map[type_node.name] = basic_type

                elif type(type_node) is UnionTypeNode:
                    # TODO:
                    pass

            # Types and rules from inherited domains will be integrated into current domain.
            empty_rules = []
            if inherit_type == InheritanceType.EXTENDS:
                domain = Domain(domain_name, filename, type_map, empty_rules, conforms,
                                extends=inherited_domain_map, logger=self.logger)
            elif inherit_type == InheritanceType.INCLUDES:
                domain = Domain(domain_name, filename, type_map, empty_rules, conforms,
                                includes=inherited_domain_map, logger=self.logger)
            else:
                domain = Domain(domain_name, filename, type_map, empty_rules, conforms, logger=self.logger)

            # Add all rules to a domain after type map is fully generated.
            for rule_node in domain_node.rules:
                head_preds = []
                for term_node in rule_node.head:
                    term = self.load_term_node(term_node, type_map)
                    pred = Predicate(term)
                    head_preds.append(pred)

                body_conjunctions = []
                for conjunction in rule_node.body:
                    body_constraints = []
                    for constraint_node in conjunction:
                        constraint = self.load_constraint_node(constraint_node, type_map)
                        body_constraints.append(constraint)
                    body_conjunctions.append(body_constraints)

                rule = Rule(head_preds, body_conjunctions)
                rules.append(rule)

            # Add all conformance nodes after type map is fully generated.
            for conformance_node in domain_node.conforms:
                body_conjunctions = []
                for conjunction in conformance_node:
                    body_constraints = []
                    for constraint_node in conjunction:
                        constraint = self.load_constraint_node(constraint_node, type_map)
                        body_constraints.append(constraint)
                    body_conjunctions.append(body_constraints)
                conforms.append(body_conjunctions)

            domain.add_rules(rules)
            domain.add_comforms(conforms)
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

            is_partial = model_sig_node.is_partial
            model_name = model_sig_node.model_name
            domain_name = model_sig_node.domain.module
            if domain_name not in domain_map:
                raise Exception('Domain %s does not exist.' % domain_name)

            # Assume inherited models are already loaded.
            inherited_refs = model_sig_node.inherited_refs
            if inherited_refs:
                for inherited_ref in inherited_refs:
                    # TODO: deal with inherited models.
                    pass

            type_map = domain_map[domain_name].type_map

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

    def load_constraint_node(self, constraint_node, type_map):
        if type(constraint_node) is TermConstraintNode:
            has_negation = constraint_node.has_negation
            term_node = constraint_node.term
            term = self.load_term_node(term_node, type_map)
            alias = constraint_node.alias
            constraint = Predicate(term, negated=has_negation)
            return constraint
        elif type(constraint_node) is BinaryConstraintNode:
            op = constraint_node.op
            left = self.load_expression_node(constraint_node.left, type_map)
            right = self.load_expression_node(constraint_node.right, type_map)
            constraint = BinaryConstraint(left, right, op)
            return constraint
        else:
            # TODO: Add other constraints
            raise Exception('Not implemented yet.')

    def load_expression_node(self, expr_node, type_map):
        if type(expr_node) is AggregationNode:
            func = expr_node.func
            set_compr_node = expr_node.set_comprehension
            tid = expr_node.tid
            default_value = expr_node.default_value
            # Turn SetComprehension node into expression.
            head_terms = [self.load_term_node(term, type_map) for term in set_compr_node.head_terms]
            constraints = [self.load_constraint_node(constraint, type_map) for constraint in set_compr_node.constraints]
            set_compr = SetComprehension(head_terms, constraints)
            return Aggregation(func, set_compr, tid, default_value)
        elif type(expr_node) is VariableTermNode:
            return self.load_term_node(expr_node, type_map)
        elif type(expr_node) is ConstantNode:
            return expr_node.constant
        else:
            raise Exception('Not implemented yet.')

    def load_term_node(self, term_node, type_map, var_type=None):
        # Turn AST node into a real FORMULA term.
        if type(term_node) is CompositeTermNode:
            type_ref_node = term_node.type
            type_name = type_ref_node.type
            ref_name = type_ref_node.ref
            if ref_name:
                sort = type_map[ref_name + '.' + type_name]
            else:
                sort = type_map[type_name]
            terms = []
            for i, subterm_node in enumerate(term_node.terms):
                # Need to find the type for variable term
                subterm_type_str = sort.types[i]
                subterm_type_ref_str = sort.refs[i]
                if subterm_type_ref_str:
                    subterm_type = type_map[subterm_type_ref_str + '.' + subterm_type_str]
                else:
                    subterm_type = type_map[subterm_type_str]
                subterm = self.load_term_node(subterm_node, type_map, subterm_type)
                terms.append(subterm)
            term = Composite(sort, terms)
            return term
        elif type(term_node) is VariableTermNode:
            return Variable('.'.join(term_node.variable), var_type)
        elif type(term_node) is ConstantNode:
            # TODO: Add type to atom node as well.
            return Atom(term_node.constant)
