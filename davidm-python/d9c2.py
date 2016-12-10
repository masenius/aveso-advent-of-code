#!/usr/bin/env python

import re

compressed = open("d9.input", "r").readline()

def get_marker(string):
    digits = re.findall(r'[\d]+', string[:10])
    return int(digits[0]), int(digits[1]), len(digits[0]) + len(digits[1]) + 3
    
def decompress(string):
    pos = 0
    total_length = 0
    while pos < len(string):
        if string[pos] == '(':
            length, repeats, marker_length = get_marker(string[pos:])
            new_string = string[pos + marker_length:pos + length + marker_length]
            total_length += repeats * decompress(new_string)
            pos += length + marker_length
        else:
            total_length += 1
            pos += 1
    return total_length
    
decompressed_length = decompress(compressed)
        
print decompressed_length
