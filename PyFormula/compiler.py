

class Compiler:
    # e.g. fact_map = {link: [[a,b], [b,c]]}
    def __init__(self, relations, rules):
        self.relation_map = {}
        self.rules = rules

        for relation in relations:
            self.relation_map[relation.name] = relation

    def compile(self, facts_map):
        for key in facts_map:
            facts = facts_map[key]
            if key in self.relation_map:
                relation = self.relation_map[key]
                for fact in facts:
                    relation.data[fact] = 1

        self.initial_evaluation()

    def initial_evaluation(self):
        # TODO: Rules need to be stratified and raise error if reduced dependency graph has cycle.
        for rule in self.rules:
            bindings_list = rule.find_match()
            for constraint in rule.head:
                facts = []
                hterm = constraint.term
                for bindings_tuple in bindings_list:
                    (bindings, bindings_count) = bindings_tuple
                    fact = hterm.propagate_bindings(bindings)
                    hterm.sort.add_fact(fact, bindings_count)

    def add_changes(self, facts_map):
        pass

    def incremental_evaluation(self):
        pass
