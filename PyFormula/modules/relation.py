from modules.term import Atom, Variable, Composite


class Relation:
    @classmethod
    def string(cls):
        return cls('string')

    @classmethod
    def integer(cls):
        return cls('integer')

    @classmethod
    def float(cls):
        return cls('float')

    '''
    All data has to be ground terms without variables.
    '''
    def __init__(self, name, labels=None, types=None):
        self.name = name
        self.labels = labels
        self.types = types
        self.data = []
        self.delta_data = []
        self.combined_data = []


    def create_ground_term(self, terms):
        return Composite(self, terms)


    def add_fact(self, tuple):
        self.delta_data.append((tuple, 1))