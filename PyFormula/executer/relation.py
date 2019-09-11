from typing import *


class BaseType:
    def __init__(self, type_name):
        self.name = type_name

    def __str__(self):
        return self.name


class UnnType(BaseType):
    def __init__(self, name: str, subtypes: List[str], ref):
        super().__init__(name)
        self.subtypes = subtypes
        self.ref = ref

    def __str__(self):
        return '_'.join([str(subtype) for subtype in self.subtypes])

    def add_reference(self, ref):
        self.name = ref + '.' + self.name
        self.ref = ref

    def has_type(self, some_type: str):
        for subtype in self.subtypes:
            if type(subtype) is UnnType:
                if subtype.has_type(some_type):
                    return True
            else:
                if subtype == some_type:
                    return True
        return False

    def has_constant(self, constant, type_map):
        """
        Check if the union type contains the constant string recursively.
        :param constant:
        :param type_map
        :return:
        """
        for subtype_str in self.subtypes:
            subtype = type_map[subtype_str]
            if type(subtype) is EnumType or type(subtype) is RangeType:
                if subtype.has_constant(constant):
                    return True, subtype
            elif type(subtype) is UnnType:
                has_match, matched_type = subtype.has_constant(constant, type_map)
                if has_match:
                    return True, matched_type
        return False, None


class RangeType(BaseType):
    def __init__(self, low, high):
        self.low = low
        self.high = high

    def __str__(self):
        return str(self.low) + '..' + str(self.high)

    def has_constant(self, constant):
        if type(constant) is int:
            if self.low <= constant <= self.high:
                return True
            else:
                return False
        else:
            raise Exception('Constant has wrong type.')


class EnumType(BaseType):
    def __init__(self, name, enums):
        super().__init__(name)
        self.enums = enums

    def __str__(self):
        return '+'.join([str(enum) for enum in self.enums])

    def has_constant(self, constant):
        if constant in self.enums:
            return True
        else:
            return False


class BuiltInType(BaseType):
    _instance_map = {}

    def __new__(cls, *args, **kwargs):
        name = args[0]
        if name not in cls._instance_map:
            cls._instance_map[name] = super().__new__(cls)
        return cls._instance_map[name]

    def __init__(self, name):
        super().__init__(name)

    def __eq__(self, other):
        return self.name == other.name

    def __hash__(self):
        return hash(self.name)

    def __str__(self):
        return self.name

    @staticmethod
    def get_types():
        return {
            'String': BuiltInType('String'),
            'Integer': BuiltInType('Integer'),
            'Boolean': BuiltInType('Boolean')
        }


class BasicType(BaseType):
    def __init__(self, name, labels=None, types=None, refs=None):
        super().__init__(name)
        self.labels = labels
        self.types = types

        # Use self.refs to denote that this type is associated with an inherited domain.
        self.refs = refs

    def __hash__(self):
        return hash(self.name)

    def __str__(self):
        type_str = self.name + '('
        for i in range(len(self.types)):
            type_str += self.labels[i] + ':' + ((self.refs[i] + '.') if self.refs[i] else '') \
                     + self.types[i] + ','
        type_str += ')'
        return type_str

    def __add__(self, other):
        """
        TODO: Overwrite operator '+' for instances of relation to create a union type
        :param other:
        :return:
        """
        pass

    def add_reference(self, ref_name):
        self.name = ref_name + '.' + self.name
        self.refs = [ref_name for i in range(len(self.refs))]
