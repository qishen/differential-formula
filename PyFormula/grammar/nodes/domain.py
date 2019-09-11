from typing import *
from enum import Enum
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
            if type(node) is BasicTypeNode or type(node) is UnionTypeNode:
                self.types.append(node)
            elif type(node) is RuleNode:
                self.rules.append(node)
            elif type(node) is list:
                # node is a list of lists of constraints
                self.conforms.append(node)

    def validate(self):
        pass


class InheritanceType(Enum):
    EXTENDS = 0
    INCLUDES = 1
    NONE = 2


class DomainSigConfigNode:
    def __init__(self, name: str , inherit_type: InheritanceType, modrefs: List[TypeRefNode]):
        self.name = name
        self.inherit_type = inherit_type
        self.modrefs = modrefs