class BaseType:
    def __init__(self, type_name):
        self.name = type_name

    def __str__(self):
        return self.name

class UnnType(BaseType):
    def __init__(self, name, subtypes):
        super().__init__(name)
        self.subtypes = subtypes


class EnumType(BaseType):
    def __init__(self, name, enums):
        super().__init__(name)
        self.enums = enums


class BuiltInType(BaseType):
    _instance_map = {}

    def __init__(self, name):
        if name not in BuiltInType._instance_map:
            super().__init__(name)
            BuiltInType._instance_map[name] = self

    def __eq__(self, other):
        return self.name == other.name

    def __hash__(self):
        return hash(self.name)

    @staticmethod
    def get_types():
        return {'String': BuiltInType('String'),
                'Integer': BuiltInType('Integer'),
                'Boolean': BuiltInType('Boolean')}


class BasicType(BaseType):
    def __init__(self, name, labels=None, types=None):
        super().__init__(name)
        self.labels = labels
        self.types = types

    def __hash__(self):
        return hash(self.name)

    def __str__(self):
        type_str = self.name + '('
        for i in range(len(self.types)):
            type_str += self.labels[i] + ':' + self.types[i].name + ','
        type_str += ')'
        return type_str

    def __add__(self, other):
        """
        TODO: Overwrite operator '+' for instances of relation to create a union type
        :param other:
        :return:
        """
        pass
