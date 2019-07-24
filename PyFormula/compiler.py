import logging
import sys
from collections import Counter


class Compiler:
    # e.g. fact_map = {link: [[a,b], [b,c]]}
    def __init__(self, relations, rules, logger_disabled=False):
        self.relation_map = {}
        self.rules = rules
        self.logger = logging.getLogger(__name__)
        self.logger.addHandler(logging.StreamHandler())
        self.logger.setLevel(logging.DEBUG)
        if logger_disabled:
            self.logger.disabled = True

        for relation in relations:
            self.relation_map[relation.name] = relation

    def print_all_facts(self):
        for name in self.relation_map:
            self.logger.debug(self.relation_map[name])

    def compile(self, facts):
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

    def print_bindings_list(self, bindings_list):
        if len(bindings_list) == 0:
            self.logger.debug('No bindings available for current rule.')
        for bindings_tuple in bindings_list:
            (bindings, count) = bindings_tuple
            bindings_str_list = []
            for (key, value) in bindings.items():
                bindings_str_list.append('[' + str(key) + ' binds to ' + str(value) + ']')
            self.logger.debug(', '.join(bindings_str_list) + ' with count ' + str(count))

    def execute_rule(self, rule):
        new_fact_counter = Counter()
        delta_rules = rule.derive_delta_rules()
        for delta_rule in delta_rules:
            self.logger.info(delta_rule)
            bindings_list = delta_rule.find_match()

            self.print_bindings_list(bindings_list)

            for bindings_tuple in bindings_list:
                (bindings, bindings_count) = bindings_tuple
                for constraint in delta_rule.head:
                    head_term = constraint.term
                    fact = head_term.propagate_bindings(bindings)

                    self.logger.debug('%s -> %s' % (fact, bindings_count))

                    # new derived fact could be a duplicate in old data set.
                    new_fact_counter.update({fact: bindings_count})
            self.logger.debug('\n')

        if rule.has_recursion:
            # Merge delta data and filter new derived data to remove duplicates
            self.merge_delta_data()
            delete_facts = []
            for fact in new_fact_counter:
                if fact in fact.sort.data:
                    delete_facts.append(fact)
            for fact in delete_facts:
                del new_fact_counter[fact]

            # Add new derived data, which are not duplicates, to delta data and execute the same rule again.
            if len(new_fact_counter) > 0:
                for fact in new_fact_counter:
                    count = new_fact_counter[fact]
                    fact.sort.add_delta_fact(fact, count)
                self.execute_rule(rule)
        else:
            # Directly add new derived terms to delta data and merge delta data into data for non-recursive rule
            for fact in new_fact_counter:
                count = new_fact_counter[fact]
                fact.sort.add_delta_fact(fact, count)

    def add_changes(self, changes):
        for fact in changes:
            count = changes[fact]
            fact.relation.add_delta_fact(fact, count)

        for rule in self.rules:
            self.execute_rule(rule)
