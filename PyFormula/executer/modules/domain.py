import networkx as nx
import copy

from executer.relation import *


class Domain:
    def __init__(self, domain_name, program_path, type_map, rules, conforms, includes=None, extends=None, logger=None):
        self.program_path = program_path
        self.logger = logger
        self.domain_name = domain_name
        self.includes = includes
        self.extends = extends
        self.type_map = type_map
        self.rules = rules
        self.stratified_rules = self.stratify_rules()
        self.conforms = conforms

        self.model_map = {}

        if includes:
            self.merge_inherited_types(includes)
        elif extends:
            # Extends types, rules and conformance.
            self.merge_inherited_types(extends)
            self.merge_inherited_rules(extends)

    def merge_inherited_rules(self, inherited_domain_map):
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

    def stratify_rules(self):
        idb = {}
        edb = {}
        edb_only = {}

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


