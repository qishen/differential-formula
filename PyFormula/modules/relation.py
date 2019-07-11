from collections import ChainMap, Counter


class CounterChainMap(ChainMap):
    """
    Extends collections.ChainMap to create a special chained map that if multiple
    maps have the same key, CounterChainMap will combine all values and return the sum.
    Note that CounterChainMap only supports maps whose values can be added and any direct
    manipulation on chain map should be prohibited.

    Another option is collections.Counter,
    combined = Counter(data) + Counter(delta_data)
    """
    def __getitem__(self, key):
        count = 0
        has_key = False
        for mapping in self.maps:
            if key in mapping:
                count += mapping[key]
                has_key = True
        if has_key:
            return count
        else:
            raise KeyError(key)


class SetDiffMap:
    """
    Only compare the first two mappings and skip other mappings.
    """
    def __init__(self, first, second):
        self.first = first
        self.second = second

    def __missing__(self, key):
        raise KeyError(key)

    def __iter__(self):
        list = []
        for key in self.first:
            if key in self.first and key not in self.second:
                list.append(key)
        return iter(list)

    def __getitem__(self, key):
        if key in self.first and key in self.second:
            raise KeyError(key)
        elif key in self.first and key not in self.second:
            return 1
        else:
            raise KeyError(key)


class Relation:
    # Some built-in basic sorts as defined as static singleton variables.
    string = None
    integer = None
    float = None
    _instance_map = {}

    def __str__(self):
        output = '------------ Model facts on Relation %s ---------------\n' % self.name
        output += '--- Data ---\n'
        for fact in self.data:
            output += str(fact) + ' -> ' + str(self.data[fact]) + '\n'
        output += '--- Delta Data ---\n'
        for fact in self.delta_data:
            output += str(fact) + ' -> ' + str(self.delta_data[fact]) + '\n'
        return output

    '''
    All data has to be ground terms without variables.
    '''
    def __init__(self, name, labels=None, types=None):
        self.name = name
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
        pass


if __name__ == '__main__':
    link = Relation('link', ['src', 'dst'], ["string", "string"])
    link.data['hello'] = 1
    link.delta_data['world'] = 2
    print(link.data)
    print(link.delta_data)
    print(link.combined_data)
    for i in link.combined_data:
        print(i, link.combined_data[i])

    d1 = {'hello': 1, 'world': 2}
    d2 = {'hello': 3, 'hi': 4}
    c = CounterChainMap(d1, d2)
    print(c['hello'])

    s = SetDiffMap(d1, d2)
    for key in s:
        print(key)