

class Compiler:
    # e.g. fact_map = {link: [[a,b], [b,c]]}
    def __init__(self, relations, rules):
        self.relation_map = {}
        self.rules = rules

        for relation in relations:
            self.relation_map[relation.name] = relation

    def compile(self, facts):
        for fact in facts:
            fact.relation.data[fact] = 1

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

    def merge_delta_data(self):
        """
        Merge delta data dict into data dict after all rules are executed.
        :return:
        """
        for name in self.relation_map:
            relation = self.relation_map[name]
            relation.merge_delta_into_data()

    def add_changes(self, changes):
        for fact in changes:
            count = changes[fact]
            fact.relation.add_delta_fact(fact, count)

        for rule in self.rules:
            delta_rules = rule.derive_delta_rules()
            for delta_rule in delta_rules:
                bindings_list = delta_rule.find_match()
                for bindings_tuple in bindings_list:
                    (bindings, bindings_count) = bindings_tuple
                    for constraint in delta_rule.head:
                        hterm = constraint.term
                        fact = hterm.propagate_bindings(bindings)
                        # new derived fact could be a duplicate in old data set.
                        hterm.sort.add_delta_fact(fact, bindings_count)

        #self.merge_delta_data()
