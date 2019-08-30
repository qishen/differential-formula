from typing import *


class EnumCnstBaseNode:
    def __init__(self):
        pass


class EnumNode:
    def __init__(self, enum_list: List[EnumCnstBaseNode]):
        self.items = enum_list


class EnumRangeCnstNode(EnumCnstBaseNode):
    def __init__(self, low_str, high_str):
        self.low = low_str
        self.high = high_str


class EnumCnstNode(EnumCnstBaseNode):
    def __init__(self, constant):
        self.constant = constant
