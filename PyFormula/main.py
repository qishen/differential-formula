from modules.rule import Rule
from modules.relation import Relation
from modules.term import Atom, Variable, Composite
from modules.constraint import PredType, Predicate
from compiler import Compiler


'''
Define some relations: link, hop and tri_hop
'''
link = Relation('link', ['src', 'dst'], ["string", "string"])
hop = Relation('hop', ['src', 'dst'], ["string", "string"])
tri_hop = Relation('tri_hop', ['src', 'dst'], ["string", "string"])
relations = [link, hop, tri_hop]

'''
Define predicates used in rules and all of them are in original form
'''
string_sort = Relation.string()
link_x_z_term = Composite(link, [Variable('X', string_sort), Variable('Z', string_sort)])
link_z_y_term = Composite(link, [Variable('Z', string_sort), Variable('Y', string_sort)])
hop_x_y_term = Composite(hop, [Variable('X', string_sort), Variable('Y', string_sort)])
hop_x_z_term = Composite(hop, [Variable('X', string_sort), Variable('Z', string_sort)])
tri_hop_x_y_term = Composite(tri_hop, [Variable('X', string_sort), Variable('Y', string_sort)])

link_x_y = Predicate(link_x_z_term, PredType.ORIGINAL, False)
link_z_y = Predicate(link_z_y_term, PredType.ORIGINAL, False)
hop_x_y = Predicate(hop_x_y_term, PredType.ORIGINAL, False)
hop_x_z = Predicate(hop_x_z_term, PredType.ORIGINAL, False)
tri_hop_x_y = Predicate(tri_hop_x_y_term, PredType.ORIGINAL, False)

'''
Rules composed by predicates
'''
hop_rule = Rule([hop_x_y], [link_x_y, link_z_y])
tri_hop_rule = Rule([tri_hop_x_y], [hop_x_z, link_z_y])
rules = [hop_rule, tri_hop_rule]


'''
Randomly creates some facts for relations
'''
link_facts = [['a', 'b'], ['a', 'd'], ['d', 'c'], ['b', 'c'], ['c', 'h'], ['f', 'g']]
link_facts = [link.create_ground_term([Atom(t[0]), Atom(t[1])]) for t in link_facts]
facts_map = {'link': link_facts}


compiler = Compiler(relations, rules)
compiler.compile(facts_map)

for t in link.data:
    print(str(t[0]), t[1])

rules = hop_rule.derive_delta_rules()
for rule in rules:
    print(rule)

