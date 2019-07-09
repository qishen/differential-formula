import more_itertools


class Relation:
    # Some built-in basic sorts as defined as static singleton variables.
    string = None
    integer = None
    float = None

    '''
    All data has to be ground terms without variables.
    '''
    def __init__(self, name, labels=None, types=None):
        self.name = name
        self.labels = labels
        self.types = types
        self.data = []
        self.delta_data = []
        self.combined_data = more_itertools.flatten([self.data, self.delta_data])

    def __new__(cls, *args, **kwargs):
        """
        Create singletons only for built-in basic sorts.
        """
        if args[0] == 'string':
            if not cls.string:
                cls.string = super().__new__(cls)
            return cls.string
        elif args[0] == 'integer':
            if not cls.integer:
                cls.integer = super().__new__(cls)
            return cls.integer
        elif args[0] == 'float':
            if not args[0] == 'float':
                cls.float = super().__new__(cls)
            return cls.float
        else:
            return super().__new__(cls)

    def add_fact(self, tuple):
        self.delta_data.append((tuple, 1))


if __name__ == '__main__':
    link = Relation('link', ['src', 'dst'], ["string", "string"])
    link.data += [1,2,3]
    link.delta_data += [4,5,6]
    print(link.data)
    print(link.delta_data)
    print(link.combined_data)
    for i in link.combined_data:
        print(i)