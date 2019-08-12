class EnumNode:
    def __init__(self, enum_list):
        self.items = enum_list


class EnumRangeCnstNode:
    def __init__(self, low_str, high_str):
        self.low = low_str
        self.high = high_str


class EnumCnstNode:
    def __init__(self, constant):
        self.constant = constant
