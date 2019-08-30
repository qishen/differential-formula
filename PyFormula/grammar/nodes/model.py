class ModelNode:
    def __init__(self, domain_sig, model_fact_list):
        self.domain_sig = domain_sig
        self.fact_list_node = model_fact_list


class ModelFactListNode:
    def __init__(self, alias_map, facts):
        self.alias_map = alias_map
        self.facts = facts


class ModelSigConfigNode:
    def __init__(self, is_partial, model_name, domain_info, inherited_refs):
        self.is_partial = is_partial
        self.model_name = model_name
        self.domain = domain_info
        self.inherited_refs = inherited_refs
