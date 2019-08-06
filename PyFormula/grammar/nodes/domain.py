from grammar.nodes.type import *
from grammar.nodes.rule import *


class DomainNode:
    def __init__(self, sig_node, nodes):
        self.domain_sig = sig_node

        # Three types of domain definitions
        self.types = []
        self.rules = []
        self.conforms = []

        # models belong to this domain
        self.model_map = {}

        for node in nodes:
            if type(node) is TypeNode or type(node) is UnionTypeNode:
                self.types.append(node)
            elif type(node) is RuleNode:
                self.rules.append(node)


class DomainSigConfigNode:
    def __init__(self, name, inherit_type, modrefs):
        self.name = name
        self.inherit_type = inherit_type
        self.modrefs = modrefs
