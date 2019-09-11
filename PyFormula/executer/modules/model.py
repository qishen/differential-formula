from collections import Counter
from executer.modules.domain import Domain
from executer.index import *
from executer.relation import *


class Model:
    def __init__(self, model_name, domain: Domain, facts=[]):
        self.logger = domain.logger
        self.model_name = model_name
        self.domain = domain
        self.type_index_map = {}
        self.initial_facts = facts

        # Create new index for each type defined in domain.
        for type_name in self.domain.type_map:
            basic_type = self.domain.type_map[type_name]
            if type(basic_type) is BasicType:
                type_index = TermIndex(basic_type)
                self.type_index_map[basic_type.name] = type_index

    def add_changes(self, changes):
        for fact in changes:
            count = changes[fact]
            index = self.type_index_map[fact.sort.name]
            index.add_delta_fact(fact, count)

        for cluster in self.domain.stratified_rules:
            for rule in cluster:
                self.execute_rule(rule)

    def compile(self):
        """
        Initial compilation will treat facts as changes to empty dataset
        and incrementally execute all rules.
        :param facts:
        :return:
        """
        changes = {}
        for fact in self.initial_facts:
            changes[fact] = 1

        self.add_changes(changes)
        self.merge_delta_data()

    def merge_delta_data(self):
        """
        Merge delta data dict into data dict after all rules are executed.
        :return:
        """
        for index in self.type_index_map.values():
            index.merge_delta_into_data()

    def insert_delta_facts(self, facts_dict):
        """
        Add every fact into delta data section of its own relation with count.
        :param facts_dict:
        :return:
        """
        for fact in facts_dict:
            count = facts_dict[fact]
            self.type_index_map[fact.sort.name].add_delta_fact(fact, count)

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

            bindings_counter = delta_rule.find_match(self.type_index_map)

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
                if fact not in self.type_index_map[fact.sort].data:
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



