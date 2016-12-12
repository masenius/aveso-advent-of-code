#!/usr/bin/env python

import re
from collections import defaultdict

f = open("d10.input","r")

transactions = []
bots = defaultdict(list)
bins = defaultdict(list)

for instruction in f.readlines():
    if 'value' in instruction:
        value, bot = map(int, re.findall(r'[\d]+', instruction))
        bots[bot].append(value)
    else:
        instruction = instruction.split()
        giver = int(instruction[1])
        low_type = instruction[5]
        low = int(instruction[6])
        high_type = instruction[-2]
        high = int(instruction[-1])
        transactions.append([giver, low_type, low, high_type, high])

running = True
while running:
    for bot, values in bots.items():
        if len(values) == 2:
            for transaction in transactions:
                giver, low_type, low, high_type, high = transaction
                if giver == bot:
                    break
                    
            minimum, maximum = min(bots[giver]), max(bots[giver])
            bots[giver] = []
            
            if low_type == 'output':
                bins[low].append(minimum)
            else:
                bots[low].append(minimum)
            
            if high_type == 'output':
                bins[high].append(maximum)
            else:
                bots[high].append(maximum)
                
            if len(bins[0]) > 0 and len(bins[1]) > 0 and len(bins[2]) > 0:
                print bins[0][0] * bins[1][0] * bins[2][0]
                running = False
                break
            
            break