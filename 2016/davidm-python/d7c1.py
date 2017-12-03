#!/usr/bin/env python

import re

f = open("d7.input", 'r')
valid = 0


def has_abba(string):
    for i in range(0, len(string) - 3):
        if string[i] != string[i + 1] and string[i] == string[i + 3] and string[i + 1] == string[i + 2]:
            return True
    return False


def check_list_for_abbas(list_):
    for string in list_:
        if has_abba(string):
            return True
    return False


for ip in f.readlines():
    non_hypernets = re.findall(r'[\w]+\[|\][\w]+', ip)
    hypernets = re.findall(r'\[[\w]+\]', ip)
    if check_list_for_abbas(non_hypernets) and check_list_for_abbas(hypernets) is False:
        valid += 1

print valid
