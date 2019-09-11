import networkx as nx
import copy

from executer.relation import *
from executer.rule import *


class Domain:
    def __init__(self, domain_name, program_path, type_map, rules, conforms, includes=None, extends=None, logger=None):
        self.program_path = program_path
        self.logger = logger
        self.domain_name = domain_name
        self.includes = includes
        self.extends = extends
        self.type_map = type_map
        self.original_rules = rules
        self.disjunction_free_rules = []
        self.stratified_rules = []
        self.conforms = conforms
        self.model_map = {}

        if includes:
            # Only includes types.
            self.merge_inherited_types(includes)
        elif extends:
            # Extends types, rules and conformance.
            self.merge_inherited_types(extends)
            self.merge_inherited_rules(extends)

        self.compile()

    def validate(self):
        # Validate rules and change variable to value of EnumType as parser does not know the semantics of each symbol.
        pass

    def compile(self):
        # Stratify disjunction free rules in which body part is only a list of constraints.
        self.disjunction_free_rules = self.convert_to_disjunction_free_rules()
        self.stratified_rules = self.stratify_rules()

    def add_rules(self, rules):
        self.original_rules += rules
        self.compile()

    def add_comforms(self, rules):
        pass

    def merge_inherited_rules(self, inherited_domain_map):
        # TODO:
        pass

    def merge_inherited_types(self, inherited_domain_map):
        # Can be either original domain name or rename
        for name in inherited_domain_map:
            domain = inherited_domain_map[name]
            # Directly extends or includes other domains
            if name == domain.domain_name:
                self.type_map.update(domain.type_map)
            else:
                # Inheritance with rename.
                rename = name
                for type_name in domain.type_map:
                    if type(domain.type_map[type_name]) is not BuiltInType:
                        new_type = copy.copy(domain.type_map[type_name])
                        new_type.add_reference(rename)
                        self.type_map[new_type.name] = new_type

    def add_model(self, name, model):
        self.model_map[name] = model

    def convert_to_disjunction_free_rules(self):
        new_rules = []
        for rule in self.original_rules:
            head = rule.head
            for disjunction in rule.body:
                new_rule = Rule(head, [disjunction])
                new_rules.append(new_rule)
        return new_rules

    def stratify_rules(self):
        """
        Only apply to a set of disjunction free rules that only
        has one disjunction of conjunctions like [[c1, c2 ... cn]]
        :return:
        """
        idb = {}
        edb = {}

        # All predicates in head belong to IDB, the rest of preds belong to EDB
        for rule in self.disjunction_free_rules:
            for c in rule.head:
                if c.term.sort not in idb:
                    idb[c.term.sort] = [rule]
                else:
                    if rule not in idb[c.term.sort]:
                        idb[c.term.sort].append(rule)

        for rule in self.disjunction_free_rules:
            for c in rule.body[0]:
                if type(c) is Predicate:
                    body_sort = c.term.sort
                    if body_sort not in idb:
                        if body_sort not in edb:
                            edb[body_sort] = [rule]
                        else:
                            if rule not in edb[body_sort]:
                                edb[body_sort].append(rule)
                else:
                    #TODO: other constraints.
                    pass

        dg = nx.DiGraph()
        for rule in self.disjunction_free_rules:
            for hc in rule.head:
                head_sort = hc.term.sort
                for bc in rule.body[0]:
                    if type(bc) is Predicate:
                        body_sort = bc.term.sort
                        if body_sort in idb:
                            if bc.negated:
                                dg.add_edge(body_sort, head_sort, negated=True)
                            else:
                                dg.add_edge(body_sort, head_sort, negated=False)
                    else:
                        #TODO: Other type of constraints
                        pass

        cg = nx.condensation(dg)
        mapping = cg.graph['mapping']

        rule_clusters = []

        '''
        Dependency graph may be empty if some IDB are not in graph and 
        a rule with only EDB in body can be left out.
        '''
        rule_cluster = []
        for sort in idb:
            if sort not in mapping:
                rule_cluster += idb[sort]
        rule_clusters.append(rule_cluster)

        for cluster_id in nx.topological_sort(cg):
            rule_cluster = []
            for node in mapping:
                if mapping[node] == cluster_id:
                    rule_cluster += idb[node]
            rule_clusters.append(rule_cluster)

        return rule_clusters

    def transform_into_magicset(self):
        pass


