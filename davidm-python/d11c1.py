#!/usr/bin/env python

import re
from collections import deque


visited_states = []

def all_on_top(states):
    for state in states:
        pairs = state[1]
        for pair in pairs:
            if pair[0] != 3 or pair[1] != 3:
                return False
    return True

def unvisited(state):
    h = hash(str(state))
    if h in visited_states:
        return False
    visited_states.append(h)
    return True


def valid(state):
    pairs = list(state[1])
    check_pairs = []
    
    # If pair is on same floor, ignore
    for pair in pairs:
        if pair[0] != pair[1]:
            check_pairs.append(pair)
    
    for i in range(len(pairs)):
        for j in range(len(check_pairs)):
            if i != j and pairs[i][0] == check_pairs[j][1]:
                return False
    
    return True
    

def get_valid_states(state):
    elevator, pairs = state

    # Generatre possible floors
    possible_floors = []
    if elevator > 0:
        possible_floors.append(-1)
    if elevator < 3:
        possible_floors.append(1)
    
    # Generate all possible states
    possible_states = []
    for floor in possible_floors:
        for i in range(len(pairs)):
            for j in range(2):
                pair = list(pairs[i])
                if pair[j] == elevator and pair[j] + floor >= 0 and pair[j] + floor <= 3:
                    pair[j] += floor
                    new_pairs = pairs[:i] + [pair] + pairs[i+1:]
                    possible_states.append([elevator + floor, new_pairs])
                    for k in range(len(new_pairs)):
                        for l in range(2):
                            # Make sure the items can be in the elevator together
                            if i == j and k == l or j == 1 and l == 0 or l == 1 and j == 0:
                                continue
                            else:
                                pair2 = list(new_pairs[k])
                                if pair2[l] == elevator and pair2[l] + floor >= 0 and pair2[l] + floor <= 3:
                                    pair2[l] += floor
                                    new_pairs2 = new_pairs[:k] + [pair2] + new_pairs[k+1:]
                                    possible_states.append([elevator + floor, new_pairs2])
    
    # Remove invalid states                
    valid_states = []
    for state in possible_states:
        if valid(state):
            valid_states.append(state)
    
    # Remove already visited states
    unvisited_states = []
    for state in valid_states:
        if unvisited(state):
            unvisited_states.append(state)
            
    return unvisited_states
            

f = open("d11.input", "r")

state = [0, []]
queue = deque()
distance = 0

# Read input and create root state
i = 0
items = []
for floor in f.readlines():
    for item in re.findall(r'a\s([a-z]+)(?:-[a-z]+)?\s([a-z]+)', floor):
        items.append([i] + list(item))
    i += 1

for brother in items:
    for sister in items:
        if brother[1] == sister[1] and brother[2] == 'generator' and sister[2] == 'microchip':
                state[1].append([brother[0], sister[0]])

queue.append(state)

while True:
    distance += 1
    states = []
    
    while len(queue) > 0:
        states = states + get_valid_states(queue.popleft())
        if len(queue) % 100 == 0:
            print len(queue)
    
    if all_on_top(states):
        print distance
        break
    
    queue.extend(states)
    
    print distance, len(visited_states)
        
        
    
        
        