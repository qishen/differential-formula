from executer.modules.domain import Domain
from executer.index import *
from executer.relation import *


class Model:
    def __init__(self, model_name, domain: Domain):
        self.model_name = model_name
        self.domain = domain
        self.type_index_map = {}
        self.initialize()

    def initialize(self):
        for type_name in self.domain.type_map:
            basic_type = self.domain.type_map[type_name]
            if type(basic_type) is BasicType:
                type_index = TermIndex(basic_type)
                self.type_index_map[basic_type] = type_index
