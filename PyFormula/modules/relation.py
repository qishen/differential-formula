import more_itertools


class Relation:
    # Some built-in basic sorts as defined as static singleton variables.
    string = None
    integer = None
    float = None
    _instance_map = {}

    '''
    All data has to be ground terms without variables.
    '''
    def __init__(self, name, labels=None, types=None):
        self.name = name
        self.labels = labels
        self.types = types
        self.data = {}
        self.delta_data = {}
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

    def add_fact(self, fact):
        if fact in self.data:
            self.data[fact] += 1
        else:
            self.data[fact] = 1

    def add_delta_fact(self, fact):
        if fact in self.data:
            self.delta_data[fact] += 1
        else:
            self.delta_data[fact] = 1


if __name__ == '__main__':
    link = Relation('link', ['src', 'dst'], ["string", "string"])
    link.data['hello'] = 1
    link.delta_data['world'] = 2
    print(link.data)
    print(link.delta_data)
    print(link.combined_data)
    for i in link.combined_data:
        print(i, link.combined_data[i])