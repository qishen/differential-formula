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

        # if the sum is 0 then skip the key.
        if has_key and count is not 0:
            return count
        else:
            raise KeyError(key)

    def __iter__(self):
        list = []
        for item in super().__iter__():
            try:
                list.append(item)
                _ = self.__getitem__(item)
            except KeyError:
                list.remove(item)
        return iter(list)

    def __len__(self):
        count = 0
        for item in super().__iter__():
            if item in self:
                try:
                    count += 1
                    _ = self.__getitem__(item)
                except KeyError:
                    count -= 1
        return count


class SetDiffMap:
    """
    Only compare the first two mappings and skip other mappings. The second map should not have more
    keys than the first map.
    1. if a key exists in both map, then key does not exist in SetDiffMap.
    2. if a key in first map but not second map, then key exists with count 1 in SetDiffMap
    3. otherwise does not exist anyway.
    """
    def __init__(self, first, second):
        self.first = first
        self.second = second

    def __missing__(self, key):
        raise KeyError(key)

    def __iter__(self):
        l = []
        for k in self.first:
            if k in self.first and k not in self.second:
                l.append(k)
        return iter(l)

    def __len__(self):
        count = 0
        for k in self.first:
            if k in self.first and k not in self.second:
                count += 1
        return count

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

        ''' Negated data set '''
        self.negated_data = {}
        self.delta_negated_data = {}
        self.combined_negated_data = CounterChainMap(self.negated_data, self.delta_negated_data)

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
    d2 = {'hello': 3, 'world': -2, 'hi': 4}
    c = CounterChainMap(d1, d2)

    print(c['hello'])
    print(c['hi'])
    print(len(c))

    for key in c:
        print(key)

    d3 = {'hello': 1, 'world': 3}
    d4 = {'world': 2, 'bug': 4}
    s = SetDiffMap(d3, d4)
    print(len(s))
    for key in s:
        print(key)
