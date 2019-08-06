from utils.data_structures import CounterChainMap, SetDiffMap


class Relation:
    def __init__(self, type_name):
        self.name = type_name


class UnnType(Relation):
    def __init__(self, name, subtypes):
        super().__init__(name)
        self.subtypes = subtypes


class EnumType(Relation):
    def __init__(self, name, enums):
        super().__init__(name)
        self.enums = enums


class BuiltInType(Relation):
    def __init__(self, name):
        super().__init__(name)


class BasicType(Relation):
    # Some built-in basic sorts as defined as static singleton variables.
    string = None
    integer = None
    float = None
    _instance_map = {}

    '''
    All data has to be ground terms without variables.
    '''
    def __init__(self, name, labels=None, types=None):
        super().__init__(name)
        self.labels = labels
        self.types = types
        self.data = {}
        self.delta_data = {}
        self.combined_data = CounterChainMap(self.data, self.delta_data)

        ''' 
        [optimized_delta_data] = set([delta_data]) - set([data]) 
        new derived [delta_data] may contain duplicates compared with [data],
        then those duplicates will not contribute to new [delta_data] in rule
        execution on next stratum, but we still have to keep an accurate
        count of duplicates to maintain incremental views because multiple rules
        with expansion from initial facts can lead to the generation of same fact,
        so when initial facts are changed, the count of duplicates tell us if
        any other rule has derivation to still hold it.
        '''
        self.optimized_delta_data = SetDiffMap(self.combined_data, self.data)

        ''' Negated data set '''
        self.negated_data = {}
        self.delta_negated_data = {}
        self.combined_negated_data = CounterChainMap(self.negated_data, self.delta_negated_data)

    def __hash__(self):
        return hash(self.name)

    def __str__(self):
        def get_printable_data(name, data):
            string = ''
            if len(data) > 0:
                string = '--- %s ---\n' % name
                for fact in data:
                    string += str(fact) + ' -> ' + str(data[fact]) + '\n'
            return string

        output = '------------ Model facts on Relation %s ---------------\n' % self.name

        for t in [('Data', self.data), ('Delta Data', self.delta_data), \
                  ('Negated Data', self.negated_data), ('Delta Negated Data', self.delta_negated_data)]:
            data_str = get_printable_data(t[0], t[1])
            output += data_str

        return output

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

    def __add__(self, other):
        """
        TODO: Overwrite operator '+' for instances of relation to create a union type
        :param other:
        :return:
        """
        pass

    def add_fact(self, fact, count=1):
        if fact in self.data:
            self.data[fact] += count
        else:
            self.data[fact] = count

    def add_delta_fact(self, fact, count):
        if fact in self.delta_data:
            self.delta_data[fact] += count
        else:
            self.delta_data[fact] = count

    def merge_delta_into_data(self):
        for fact in self.delta_data:
            if fact in self.data:
                self.data[fact] += self.delta_data[fact]
                if self.data[fact] == 0:
                    del self.data[fact]
            else:
                self.data[fact] = self.delta_data[fact]
        # Delete all delta_data after merge.
        self.delta_data.clear()

    def facts_count(self):
        count = 0
        for fact in self.combined_data:
            count += self.combined_data[fact]
        return count
