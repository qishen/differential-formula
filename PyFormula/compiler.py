

class Compiler:
    # e.g. fact_map = {link: [[a,b], [b,c]]}
    def __init__(self, relations, rules):
        self.relation_map = {}
        self.rules = rules

        for relation in relations:
            self.relation_map[relation.name] = relation

    def print_all_facts(self):
        for name in self.relation_map:
            print(self.relation_map[name])

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
            print('No bindings available for current rule.')
        for bindings_tuple in bindings_list:
            (bindings, count) = bindings_tuple
            bindings_str_list = []
            for (key, value) in bindings.items():
                bindings_str_list.append('[' + str(key) + ' binds to ' + str(value) + ']')
            print(', '.join(bindings_str_list) + ' with count ' + str(count))
        print('\n')

    def add_changes(self, changes):
        for fact in changes:
            count = changes[fact]
            fact.relation.add_delta_fact(fact, count)

        for rule in self.rules:
            delta_rules = rule.derive_delta_rules()
            for delta_rule in delta_rules:
                print(delta_rule)
                bindings_list = delta_rule.find_match()

                self.print_bindings_list(bindings_list)

                for bindings_tuple in bindings_list:
                    (bindings, bindings_count) = bindings_tuple
                    for constraint in delta_rule.head:
                        hterm = constraint.term
                        fact = hterm.propagate_bindings(bindings)

                        print('%s -> %s' % (fact, bindings_count))

                        # new derived fact could be a duplicate in old data set.
                        hterm.sort.add_delta_fact(fact, bindings_count)


