import networkx as nx
from collections import Counter


class Domain:
    def __init__(self, domain_name, type_map, rules, conforms, includes=None, extends=None, logger=None):
        self.logger = logger
        self.domain_name = domain_name
        self.includes = includes
        self.extends = extends
        self.type_map = type_map
        self.rules = rules
        self.stratified_rules = self.stratify_rules()
        self.conforms = conforms

        self.model_map = {}

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


