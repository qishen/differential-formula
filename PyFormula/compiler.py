import logging

from antlr4 import *

from executer.modules.model import *
from executer.rule import *
from grammar.gen.FormulaLexer import FormulaLexer
from grammar.gen.FormulaParser import FormulaParser
from grammar.nodes.aggregation import *
from grammar.nodes.constraint import *
from grammar.nodes.domain import *
from grammar.nodes.enum import *
from grammar.nodes.expression import *
from grammar.nodes.term import *
from grammar.nodes.type import *
from grammar.visitor import ExprVisitor

from ddengine import DDExecuter, Atom, Variable, Composite


class Compiler:
    # e.g. fact_map = {link: [[a,b], [b,c]]}
    def __init__(self, logger_disabled=False, ddengine=True):
        self.programs = {}
        self.logger = logging.getLogger(__name__)
        if not self.logger.handlers:
            self.logger.addHandler(logging.StreamHandler())
        self.logger.setLevel(logging.DEBUG)
        if logger_disabled:
            self.logger.disabled = True

        self.ddengine_enabled = ddengine

    def clear_all(self):
        self.programs.clear()

    def print_all_facts(self, model_name):
        model = self.find_model_by_name(model_name)
        for index in model.type_index_map.values():
            self.logger.debug(index)

    def execute_model(self, model_name):
        model = self.find_model_by_name(model_name)
        if model:
            if self.ddengine_enabled:
                executer = DifferentialDataflowExecuter(model)
            else:
                executer = IncrementalExecuter(model)
            executer.compile()
        else:
            self.logger.info('Cannot find the model with name %s' % model_name)

    def generate_model(self, domain_name, model_name):
        domain = self.find_domain_by_name(domain_name)
        model = Model(model_name, domain)
        domain.model_map[model] = model
        return model

    def make_changes_and_execute(self, model_name, changes):
        model = self.find_model_by_name(model_name)
        if model:
            if self.ddengine_enabled:
                executer = DifferentialDataflowExecuter(model)
            else:
                executer = IncrementalExecuter(model)
            executer.add_changes(changes)
        else:
            self.logger.info('Cannot find the model with name %s' % model_name)

    def find_domain_by_name(self, name):
        for program in self.programs:
            domain_map = self.programs[program]
            for domain in domain_map.values():
                if domain.domain_name == name:
                    return domain
        raise Exception('No domain with name %s is found.' % name)

    def find_model_by_name(self, name):
        for program in self.programs:
            domain_map = self.programs[program]
            for domain in domain_map.values():
                for model in domain.model_map.values():
                    if model.model_name == name:
                        return model
        raise Exception('No model with name %s is found.' % name)

    def parse(self, file_str=None, filename=None):
        if file_str:
            filename = 'Program-' + str(hash(file_str)) + '.4ml'
            stream = InputStream(file_str)
        else:
            stream = FileStream(filename)
        lexer = FormulaLexer(stream)
        stream = CommonTokenStream(lexer)
        parser = FormulaParser(stream)
        program = parser.program()
        domain_map, model_map = self.load_program(program, filename)
        self.programs[filename] = domain_map

    def load_program(self, program, filename):
        visitor = ExprVisitor()
        visitor.visit(program)
        domain_map = {}
        model_map = {}

        for name in visitor.domains:
            domain_node = visitor.domains[name]
            domain_sig = domain_node.domain_sig
            domain_name = domain_sig.name
            modref_nodes = domain_sig.modrefs
            inherit_type = domain_sig.inherit_type

            # Suppose inherited domains are always loaded first, then map renamed id to domain.
            inherited_domain_map = {}
            if modref_nodes:
                for modref_node in modref_nodes:
                    # Need to find the domain map in another program.
                    if modref_node.path is not None:
                        if modref_node.path not in self.programs:
                            raise Exception('Wrong path and cannot find the program with path %s' % modref_node.path)
                        else:
                            if modref_node.module in self.programs[modref_node.path]:
                                domain = self.programs[modref_node.path][modref_node.module]
                            else:
                                raise Exception('Domain %s is not defined in program %s' % modref_node.module, modref_node.path)
                    else:
                        if modref_node.module in domain_map:
                            domain = domain_map[modref_node.module]
                        else:
                            raise Exception('Domain %s is not defined in current program' % modref_node.module)

                    if modref_node.rename:
                        # Domain definition with rename, e.g. domain IsoDAGs extends Left::DAGs, Right::DAGs
                        inherited_domain_map[modref_node.rename] = domain
                    else:
                        # Domain definition without rename, then use domain name as the key in map
                        inherited_domain_map[modref_node.module] = domain

            # Store all type and rule definitions.
            type_map = {}
            rules = []
            conforms = []

            # Add built-in types
            built_in_type_map = BuiltInType.get_types()
            for type_name in built_in_type_map:
                type_map[type_name] = built_in_type_map[type_name]

            # Types should be sorted after validation so Edge does not occur before Node.
            for type_node in domain_node.types:
                type_name = type_node.name
                if type(type_node) is BasicTypeNode:
                    types = []
                    refs = []
                    labels = type_node.labels
                    subtype_nodes = type_node.types
                    # subtype node can be either regular type or an union of types.
                    for subtype_node in subtype_nodes:
                        if type(subtype_node) is list:
                            # subtype_node can be a list of string or EnumNode e.g. Node + Edge + {NIL}
                            assigned_union_type_name, unn_type = self.load_union_node(None, subtype_node, type_map)
                            type_map[assigned_union_type_name] = unn_type
                            # Add union type name to subtype list
                            types.append(assigned_union_type_name)
                            refs.append(None)
                        elif type(subtype_node) is TypeRefNode:
                            # Need to consider domain alias like Left::DAGs and Right::DAGs
                            types.append(subtype_node.type)
                            refs.append(subtype_node.ref)
                    basic_type = BasicType(type_name, labels=labels, types=types, refs=refs)
                    type_map[type_name] = basic_type

                elif type(type_node) is UnionTypeNode:
                    unn_name = type_node.name
                    subtypes = type_node.subtypes
                    _, unn_type = self.load_union_node(unn_name, subtypes, type_map)
                    type_map[unn_name] = unn_type

            ''' 
            Types and rules from inherited domains will be integrated into current domain.
            A domain can extends or includes other domains but not do both of them.
            '''
            empty_rules = []
            if inherit_type == InheritanceType.EXTENDS:
                domain = Domain(domain_name, filename, type_map, empty_rules, conforms,
                                extends=inherited_domain_map, logger=self.logger)
            elif inherit_type == InheritanceType.INCLUDES:
                domain = Domain(domain_name, filename, type_map, empty_rules, conforms,
                                includes=inherited_domain_map, logger=self.logger)
            else:
                domain = Domain(domain_name, filename, type_map, empty_rules, conforms, logger=self.logger)

            ''' 
            Only add all rules to a domain after type map is fully generated and when a new rule is added into
            existing rules, the transformed rules and stratification will be updated accordingly.
            '''
            for rule_node in domain_node.rules:
                head_preds = []
                for term_node in rule_node.head:
                    term = self.load_term_node(term_node, type_map, BuiltInType('Boolean'))
                    pred = Predicate(term)
                    head_preds.append(pred)

                body_conjunctions = []
                for conjunction in rule_node.body:
                    body_constraints = []
                    for constraint_node in conjunction:
                        constraint = self.load_constraint_node(constraint_node, type_map)
                        body_constraints.append(constraint)
                    body_conjunctions.append(body_constraints)

                rule = Rule(head_preds, body_conjunctions)
                rules.append(rule)

            # Add all conformance nodes after type map is fully generated.
            for conformance_node in domain_node.conforms:
                body_conjunctions = []
                for conjunction in conformance_node:
                    body_constraints = []
                    for constraint_node in conjunction:
                        constraint = self.load_constraint_node(constraint_node, type_map)
                        body_constraints.append(constraint)
                    body_conjunctions.append(body_constraints)
                conforms.append(body_conjunctions)

            domain.add_rules(rules)
            domain.add_comforms(conforms)
            domain_map[domain_name] = domain

        # Model facts should be sorted too, so alias occurs after its definition.
        for model_name in visitor.models:
            facts = []
            alias_to_fact_map = {}
            model_node = visitor.models[model_name]

            # ModelFactListNode
            model_fact_list_node = model_node.fact_list_node
            alias_map = model_fact_list_node.alias_map
            fact_nodes = model_fact_list_node.facts

            # ModelSigConfigNode
            model_sig_node = model_node.domain_sig

            is_partial = model_sig_node.is_partial
            model_name = model_sig_node.model_name
            domain_name = model_sig_node.domain.module
            if domain_name not in domain_map:
                raise Exception('Domain %s does not exist.' % domain_name)

            # Assume inherited models are already loaded.
            inherited_refs = model_sig_node.inherited_refs
            if inherited_refs:
                for inherited_ref in inherited_refs:
                    # TODO: deal with inherited models.
                    pass

            type_map = domain_map[domain_name].type_map

            for fact_node in fact_nodes:
                if fact_node in alias_map:
                    alias = alias_map[fact_node]
                else:
                    alias = None

                if type(fact_node) is CompositeTermNode:
                    fact = self.load_term_node(fact_node, type_map)
                    if alias:
                        fact.alias = alias
                        alias_to_fact_map[alias] = fact
                    facts.append(fact)

            ''' 
            Since parser has no idea of the semantics of FORMULA languge, it will treat model
            reference like variables and here compiler replaces all model reference with model 
            facts given an alias to model map. After in-place variable replacement, check_ground_term() 
            function will be invoked again to check the groundness of every term.
            '''
            for fact in facts:
                fact.replace_variables_in_place(alias_to_fact_map)

            domain = domain_map[domain_name]
            model = Model(model_name, domain, facts)
            domain.add_model(model_name, model)
            model_map[model_name] = model

        return domain_map, model_map

    def load_union_node(self, name, subtypes, type_map):
        # union is a list of strings that represent types.
        union = []
        for component in subtypes:
            if type(component) is str:
                type_str = component
                union.append(type_str)
            elif type(component) is EnumNode:
                # Create a new enum type to represent a list of constants.
                enum_items = []
                for cnst_node in component.items:
                    if type(cnst_node) is EnumCnstNode:
                        enum_items.append(cnst_node.constant.constant)
                    elif type(cnst_node) is EnumRangeCnstNode:
                        low = int(cnst_node.low)
                        high = int(cnst_node.high)
                        range_type = RangeType(low, high)
                        enum_items.append(range_type)
                assigned_enum_name = 'enum_' + '+'.join([str(enum) for enum in enum_items])
                enum_type = EnumType(assigned_enum_name, enum_items)
                type_map[assigned_enum_name] = enum_type
                union.append(assigned_enum_name)
        # Create a new union type
        if name is None:
            name = 'union_of_' + '#'.join(union)
        unn_type = UnnType(name, union, None)
        type_map[name] = unn_type
        # Add union type name to subtype list
        return name, unn_type

    def load_constraint_node(self, constraint_node, type_map):
        if type(constraint_node) is TermConstraintNode:
            has_negation = constraint_node.has_negation
            term_node = constraint_node.term
            term = self.load_term_node(term_node, type_map)
            alias = constraint_node.alias
            if alias is not None:
                term.alias = alias
            constraint = Predicate(term, negated=has_negation)
            return constraint
        elif type(constraint_node) is BinaryConstraintNode:
            op = constraint_node.op
            left = self.load_expression_node(constraint_node.left, type_map)
            right = self.load_expression_node(constraint_node.right, type_map)
            constraint = BinaryConstraint(left, right, op)
            return constraint
        elif type(constraint_node) is TypeConstraintNode:
            type_ref_node = constraint_node.type
            variable_strs = constraint_node.variable
            type_name = type_ref_node.type
            variable = Variable('.'.join(variable_strs), type_map[type_name])
            return TypeConstraint(variable, type_map[type_name])
        elif type(constraint_node) is DerivedConstantConstraintNode:
            negated = constraint_node.negated
            bool_var_str = constraint_node.variable
            bool_var = Variable(bool_var_str, BuiltInType('Boolean'))
            return DerivedConstantConstraint(negated, bool_var)
        else:
            # TODO: Add other constraints
            raise Exception('Not implemented yet.')

    def load_expression_node(self, expr_node, type_map):
        if type(expr_node) is AggregationNode:
            func = expr_node.func
            set_compr_node = expr_node.set_comprehension
            tid = expr_node.tid
            default_value = None
            # Default value could be an constant but recognized by parser as variable node without knowing the semantics
            if type(expr_node.default_value) is VariableTermNode:
                """ 
                Parser recognized it as variable but has to be converted into enum value otherwise wrong 
                argument provided.
                """
                var_str = '.'.join(expr_node.default_value.variable)
                # Need to decide the type of enum constant by checking tid
                type_str = tid.strip('#')
                if type_str in type_map:
                    var_type = type_map[type_str]
                else:
                    raise Exception('Wrong type id %s is provided.' % tid)

                if type(var_type) is EnumType or type(var_type) is RangeType:
                    if var_type.has_constant(var_str):
                        # Even the parser recognizes it as a variable, still turn it into an atom.
                        default_value = Atom(var_str, var_type)
                    else:
                        raise Exception('Wrong default value %s is provided.' % var_str)
                elif type(var_type) is UnnType:
                    # Recursively find the right type for constant.
                    has_match, matched_type = var_type.has_constant(var_str, type_map)
                    if has_match:
                        default_value = Atom(var_str, matched_type)
                    else:
                        raise Exception('Wrong default value %s is provided.' % var_str)
                else:
                    raise Exception('Default value does not conform to the defined type')
            elif type(expr_node.default_value) is CompositeTermNode:
                default_term = self.load_term_node(expr_node.default_value, type_map)
                if not default_term.is_ground_term:
                    raise Exception('Default value must be a constant or ground term.')
                default_value = default_term
            elif type(expr_node.default_value) is ConstantNode:
                default_value = expr_node.default_value.constant
            # Turn SetComprehension node into expression.
            head_terms = [self.load_term_node(term, type_map) for term in set_compr_node.head_terms]
            body = [[self.load_constraint_node(constraint, type_map) for constraint in set_compr_node.constraints]]
            set_compr = SetComprehension(head_terms, body)
            return Aggregation(func, set_compr, tid, default_value)
        elif type(expr_node) is VariableTermNode:
            return self.load_term_node(expr_node, type_map)
        elif type(expr_node) is ConstantNode:
            return expr_node.constant
        elif type(expr_node) is ArithmeticExprNode:
            left = self.load_expression_node(expr_node.left, type_map)
            right = self.load_expression_node(expr_node.right, type_map)
            op = expr_node.op
            return ArithmeticExpr(left, right, op)
        else:
            raise Exception('Not implemented yet.')

    def load_term_node(self, term_node, type_map, var_type=None):
        # Turn AST node into a real FORMULA term.
        if type(term_node) is CompositeTermNode:
            type_ref_node = term_node.type
            type_name = type_ref_node.type
            ref_name = type_ref_node.ref
            if ref_name:
                sort = type_map[ref_name + '.' + type_name]
            else:
                sort = type_map[type_name]
            terms = []
            for i, subterm_node in enumerate(term_node.terms):
                # Need to find the type for variable term
                subterm_type_str = sort.types[i]
                subterm_type_ref_str = sort.refs[i]
                if subterm_type_ref_str:
                    subterm_type = type_map[subterm_type_ref_str + '.' + subterm_type_str]
                else:
                    subterm_type = type_map[subterm_type_str]
                subterm = self.load_term_node(subterm_node, type_map, subterm_type)
                terms.append(subterm)
            term = Composite(sort, terms)
            return term
        elif type(term_node) is VariableTermNode:
            # The variable AST node contains a list of strings separated by dots.
            # Need to distinguish between variable and constant like NIL.
            var_str = '.'.join(term_node.variable)
            if type(var_type) is EnumType or type(var_type) is RangeType:
                if var_type.has_constant(var_str):
                    # Even the parser recognizes it as a variable, still turn it into an atom.
                    return Atom(var_str, var_type)
            elif type(var_type) is UnnType:
                # Recursively find the right type for constant.
                has_match, matched_type = var_type.has_constant(var_str, type_map)
                if has_match:
                    return Atom(var_str, matched_type)
            return Variable(var_str, var_type)
        elif type(term_node) is ConstantNode:
            # TODO: Add type to atom node as well if it is Enum type
            return Atom(term_node.constant)
