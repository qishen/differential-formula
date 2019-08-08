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
    def __init__(self, relations, rules, logger_disabled=False):
        self.programs = {}
        self.logger = logging.getLogger(__name__)
        if not self.logger.handlers:
            self.logger.addHandler(logging.StreamHandler())
        self.logger.setLevel(logging.DEBUG)
        if logger_disabled:
            self.logger.disabled = True

    def parse_file(self, filename):
        file_stream = FileStream(filename)

    def parse_string(self, file_str):
        fake_filename = 'Program-' + str(hash(file_str)) + '.4ml'
        input_stream = InputStream(file_str)
        lexer = FormulaLexer(input_stream)
        stream = CommonTokenStream(lexer)
        parser = FormulaParser(stream)
        program = parser.program()
        domain_map, model_map = self.load_program(program)
        self.programs[fake_filename] = domain_map

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
                alias = alias_map[fact_node]
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
            model = Model(model_name, domain)
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
        """
        Turn a term node from parser to a real term.
        :param term_node:
        :param type_map:
        :param var_type:
        :return:
        """
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

    def print_all_facts(self):
        for name in self.relation_map:
            self.logger.debug(self.relation_map[name])

    def compile(self, facts):
        """
        Initial compilation will treat facts as changes to empty dataset
        and incrementally execute all rules.
        :param facts:
        :return:
        """
        changes = {}
        for fact in facts:
            changes[fact] = 1

        self.add_changes(changes)
        self.merge_delta_data()

    def merge_delta_data(self):
        """
        Merge delta data dict into data dict after all rules are executed.
        :return:
        """
        for name in self.relation_map:
            relation = self.relation_map[name]
            relation.merge_delta_into_data()

    def print_bindings_list(self, bindings_counter):
        """
        Use default logger to print out all existing bindings of variables to terms with count.
        :param bindings_list:
        :return:
        """
        if len(bindings_counter) == 0:
            self.logger.debug('No bindings available for current rule.')
        else:
            self.logger.debug(str(bindings_counter))

    def execute_rule(self, rule):
        new_fact_counter = Counter()
        delta_rules = rule.derive_delta_rules()
        for delta_rule in delta_rules:

            self.logger.info(delta_rule)

            bindings_counter = delta_rule.find_match()

            self.print_bindings_list(bindings_counter)

            for bindings in bindings_counter:
                bindings_count = bindings_counter[bindings]
                for constraint in delta_rule.head:
                    head_term = constraint.term
                    fact = head_term.propagate_bindings(bindings)

                    self.logger.debug('%s -> %s' % (fact, bindings_count))

                    # new derived fact could be a duplicate in old data set.
                    new_fact_counter.update({fact: bindings_count})
            self.logger.debug('\n')

        ''' 
        Note that counting algorithm is not efficient for recursive rule execution and does not 
        terminate on some situations.
        '''
        if rule.has_recursion:
            ''' Merge delta data and find all new derived facts that does not exist in data.'''
            self.merge_delta_data()
            non_duplicate_facts = {}
            for fact in new_fact_counter:
                if fact not in fact.sort.data:
                    non_duplicate_facts[fact] = new_fact_counter[fact]
            ''' 
            Add all derived facts into delta data no matter if some facts are duplicates,
            if derived facts have non-duplicate facts then execute the same rule again.
            '''
            self.insert_delta_facts(new_fact_counter)
            if len(non_duplicate_facts) > 0:
                self.execute_rule(rule)
        else:
            # Directly add new derived terms to delta data and merge delta data into data for non-recursive rule
            self.insert_delta_facts(new_fact_counter)

    def insert_delta_facts(self, facts_dict):
        """
        Add every fact into delta data section of its own relation with count.
        :param facts_dict:
        :return:
        """
        for fact in facts_dict:
            count = facts_dict[fact]
            fact.sort.add_delta_fact(fact, count)

    def add_changes(self, changes):
        for fact in changes:
            count = changes[fact]
            fact.relation.add_delta_fact(fact, count)

        for cluster in self.stratified_rules:
            for rule in cluster:
                self.execute_rule(rule)


