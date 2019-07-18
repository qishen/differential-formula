import logging

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
only_tri_hop = Relation('only_tri_hop', ['src', 'dst'], ['string', 'string'])
relations = [link, hop, tri_hop, only_tri_hop]

'''
Define predicates used in rules and all of them are in original form
'''
string_sort = Relation('string')
link_x_z_term = Composite(link, [Variable('X', string_sort), Variable('Z', string_sort)])
link_z_y_term = Composite(link, [Variable('Z', string_sort), Variable('Y', string_sort)])
hop_x_y_term = Composite(hop, [Variable('X', string_sort), Variable('Y', string_sort)])
hop_x_z_term = Composite(hop, [Variable('X', string_sort), Variable('Z', string_sort)])
tri_hop_x_y_term = Composite(tri_hop, [Variable('X', string_sort), Variable('Y', string_sort)])
only_tri_hop_x_y_term = Composite(only_tri_hop, [Variable('X', string_sort), Variable('Y', string_sort)])

link_x_z = Predicate(link_x_z_term, PredType.ORIGINAL, False)
link_z_y = Predicate(link_z_y_term, PredType.ORIGINAL, False)
hop_x_y = Predicate(hop_x_y_term, PredType.ORIGINAL, False)
negated_hop_x_y = Predicate(hop_x_y_term, PredType.ORIGINAL, True)
hop_x_z = Predicate(hop_x_z_term, PredType.ORIGINAL, False)
tri_hop_x_y = Predicate(tri_hop_x_y_term, PredType.ORIGINAL, False)
only_tri_hop_x_y = Predicate(only_tri_hop_x_y_term, PredType.ORIGINAL, False)

'''
Rules composed by predicates
'''
hop_rule = Rule([hop_x_y], [link_x_z, link_z_y])
tri_hop_rule = Rule([tri_hop_x_y], [hop_x_z, link_z_y])
only_tri_hop_rule = Rule([only_tri_hop_x_y], [tri_hop_x_y, negated_hop_x_y])
rules = [hop_rule, tri_hop_rule, only_tri_hop_rule]

print('\n--- Print out all rules ---')
for rule in rules:
    print(str(rule))

'''
Randomly creates some facts for relations
'''
link_facts_raw = [['a', 'b'], ['a', 'd'], ['d', 'c'], ['b', 'c'], ['c', 'h'], ['f', 'g']]
link_facts = [Composite(link, [Atom(t[0]), Atom(t[1])]) for t in link_facts_raw]


print('\n--- Test on derived delta rules from one rule ---')
for rule in rules:
    derived_rules = rule.derive_delta_rules()
    for derived_rule in derived_rules:
        print(derived_rule)
    print('\n')

print('\n--- Test on bindings between ground term %s and variable term %s ---' % (str(link_facts[0]), str(link_x_z_term)))
bindings = link_x_z_term.get_bindings(link_facts[0])
for (key, value) in bindings.items():
    print(str(key) + ' binds to ' + str(value))

'''
print('\n--- Test on pattern match on rule %s ---' % str(hop_rule))
bindings_list = hop_rule.find_match()
for bindings_tuple in bindings_list:
    (bindings, count) = bindings_tuple
    bindings_str_list = []
    for (key, value) in bindings.items():
        bindings_str_list.append('[' + str(key) + ' binds to ' + str(value) + ']')
    print(', '.join(bindings_str_list))
    print('\n')
'''

print('\n--- Test on incremental evaluation ---')
compiler = Compiler(relations, rules)
compiler.compile(link_facts)

c1 = Composite(link, [Atom('a'), Atom('b')])
c2 = Composite(link, [Atom('d'), Atom('f')])
c3 = Composite(link, [Atom('a'), Atom('f')])
changes = {c1: -1, c2: 1, c3: 1}

print('Make some changes to existing facts: ')
for (term, count) in changes.items():
    if count > 0:
        print('Add ', term)
    else:
        print('Remove ', term)

compiler.add_changes(changes)

print(str(link))
print(str(hop))
print(str(tri_hop))
#print(str(only_tri_hop))
logger = logging.getLogger()
logger.addHandler(logging.StreamHandler())
logger.disabled = True
logger.setLevel(logging.DEBUG)
logger.info(str(only_tri_hop))
