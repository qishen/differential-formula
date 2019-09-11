from collections import Counter


class Bindings(dict):
    """
    Inherit built-in dict class with hash value computed on both keys and values,
    while default dict is not hashable.
    """
    def __init__(self, initial_bindings={}):
        self.update(initial_bindings)

    def _members(self):
        return tuple(list(self.keys()) + list(self.values()))

    def __eq__(self, other):
        if len(self) is not len(other):
            return False
        else:
            for key in self:
                if key not in other or self[key] != other[key]:
                    return False
            return True

    def __hash__(self):
        return hash(self._members())

    def __str__(self):
        bindings_str_list = []
        for (key, value) in self.items():
            bindings_str_list.append('[' + str(key) + ' binds to ' + str(value) + ']')
        return ', '.join(bindings_str_list)


class BindingsCounter(Counter):
    """
    Use Counter to remove possible duplicates when some different bindings are extended
    and results in the same new bindings
    """
    def __str__(self):
        bindings_counter_str = ''
        for bindings in self:
            bindings_counter_str += str(bindings) + ' with count ' + str(self[bindings]) + '\n'
        return bindings_counter_str
