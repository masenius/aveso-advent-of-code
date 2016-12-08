#!/usr/bin/env python

import re

f = open("d7.input", 'r')
valid = 0

def has_aba(string):
    bab = []
    for i in range(0, len(string) - 2):
        if string[i] != string[i + 1] and string[i] == string[i + 2]:
            bab.append(string[i + 1] + string[i] + string[i + 1])
    return bab
            
def check_list_for_aba(list_):
    babs = []
    for string in list_:
        babs += has_aba(string)
    return babs
    
def check_list_for_bab(list_, bab):
    for string in list_:
        if bab in string:
            return True
    return False

for ip in f.readlines():
    supernets = re.findall(r'[\w]+\[|\][\w]+', ip)
    hypernets = re.findall(r'\[[\w]+\]', ip)
    babs = check_list_for_aba(supernets)
    for bab in babs:
        if check_list_for_bab(hypernets, bab):
            valid += 1
            break

print valid