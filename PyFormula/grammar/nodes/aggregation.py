class SetComprehensionNode:
    def __init__(self, head_terms, constraints):
        self.head_terms = head_terms
        self.constraints = constraints


class AggregationNode:
    def __init__(self, func_name, set_comprehension, tid=None, default_value=None):
        self.func = func_name
        self.set_comprehension = set_comprehension
        self.tid = tid
        self.default_value = default_value
