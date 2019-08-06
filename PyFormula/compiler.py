import logging
from collections import Counter

import networkx as nx
from antlr4 import *

from grammar.visitor import ExprVisitor
from grammar.gen.FormulaLexer import FormulaLexer
from grammar.gen.FormulaParser import FormulaParser


class Compiler:
    # e.g. fact_map = {link: [[a,b], [b,c]]}
    def __init__(self, relations, rules, logger_disabled=False):
        self.domains = {}
        self.relation_map = {}
        self.rules = rules
        self.stratified_rules = self.stratify_rules()

        self.logger = logging.getLogger(__name__)
        if not self.logger.handlers:
            self.logger.addHandler(logging.StreamHandler())
        self.logger.setLevel(logging.DEBUG)
        if logger_disabled:
            self.logger.disabled = True

        for relation in relations:
            self.relation_map[relation.name] = relation

    def parse_string(self, file_str):
        input_stream = InputStream(file_str)
        lexer = FormulaLexer(input_stream)
        stream = CommonTokenStream(lexer)
        parser = FormulaParser(stream)
        program = parser.program()
        visitor = ExprVisitor()
        visitor.visit(program)
        domain_map = visitor.domains

    def parse_file(self, filename):
        file_stream = FileStream(filename)

    def stratify_rules(self):
        idb = {}
        edb = {}

        # All predicates in head belong to IDB, the rest of preds belong to EDB
        for rule in self.rules:
            for c in rule.head:
                if c.term.sort not in idb:
                    idb[c.term.sort] = [rule]
                else:
                    if rule not in idb[c.term.sort]:
                        idb[c.term.sort].append(rule)

        for rule in self.rules:
            for c in rule.body:
                body_sort = c.term.sort
                if body_sort not in idb:
                    if body_sort not in edb:
                        edb[body_sort] = [rule]
                    else:
                        if rule not in edb[body_sort]:
                            edb[body_sort].append(rule)

        dg = nx.DiGraph()
        for rule in self.rules:
            for hc in rule.head:
                head_sort = hc.term.sort
                for bc in rule.body:
                    body_sort = bc.term.sort
                    if body_sort in idb:
                        if bc.negated:
                            dg.add_edge(body_sort, head_sort, negated=True)
                        else:
                            dg.add_edge(body_sort, head_sort, negated=False)

        cg = nx.condensation(dg)
        mapping = cg.graph['mapping']

        rule_clusters = []
        for cluster_id in nx.topological_sort(cg):
            rule_cluster = []
            for node in mapping:
                if mapping[node] == cluster_id:
                    rule_cluster += idb[node]
            rule_clusters.append(rule_cluster)

        return rule_clusters

    def transform_into_magicset(self):
        pass

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
