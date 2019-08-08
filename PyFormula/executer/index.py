from utils.data_structures import CounterChainMap, SetDiffMap


class ModelIndex:
    def __init__(self, domain):
        self.domain = domain
        self.type_map = {}


class TermIndex:
    def __init__(self, basic_type):
        self.type = basic_type
        # All data has to be ground terms without variables.
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

        output = '------------ Model facts on type %s ---------------\n' % self.name

        for t in [('Data', self.data), ('Delta Data', self.delta_data), \
                  ('Negated Data', self.negated_data), ('Delta Negated Data', self.delta_negated_data)]:
            data_str = get_printable_data(t[0], t[1])
            output += data_str

        return output

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
