#!/usr/bin/env python

import re
from hashlib import md5
from collections import defaultdict


def get_triplet(h):
    triplet = re.findall(r'(.)\1\1', h)
    if triplet:
        return triplet[0]
    return False


def get_femlet(h):
    return re.findall(r'(.)\1\1\1\1', h)


# salt = 'abc'
salt = 'qzyelonm'
index = 0
triplet_index = defaultdict(list)
keys = []

while True:
    h = md5(salt + str(index)).hexdigest()

    for femlet in get_femlet(h):
        for triplet in triplet_index[femlet]:
            if index - triplet <= 1000:
                keys.append(triplet)
        triplet_index[femlet] = []

    triplet = get_triplet(h)
    if triplet:
        triplet_index[triplet].append(index)

    if len(keys) >= 64 and index - sorted(keys)[63] > 1000:
        print sorted(keys)[63]
        break

    index += 1
