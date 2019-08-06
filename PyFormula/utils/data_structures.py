from collections import ChainMap


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

    def __contains__(self, item):
        count = 0
        for mapping in self.maps:
            if item in mapping:
                count += mapping[item]
        if count == 0:
            return False
        else:
            return True

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