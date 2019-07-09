

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
                    relation.data.append((fact, 1))

    def initial_evaluation(self):
        pass

    def add_changes(self):
        pass

    def incremental_evaluation(self):
        pass