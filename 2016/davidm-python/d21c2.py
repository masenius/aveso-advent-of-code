#!/usr/bin/env python

from itertools import permutations

f = open('d21.input', 'r')
instructions = []

for line in f.readlines():
    line = line.split()
    instructions.append(line)


def swap_position(string, pos1, pos2):
    char = string[pos1]
    string[pos1] = string[pos2]
    string[pos2] = char


def swap_letter(string, letter1, letter2):
    letter1_index = []
    letter2_index = []

    for i in range(len(string)):
        if string[i] is letter1:
            letter1_index.append(i)
        elif string[i] is letter2:
            letter2_index.append(i)

    for i in range(len(string)):
        if i in letter2_index:
            string[i] = letter1
        elif i in letter1_index:
            string[i] = letter2


def rotate(string, direction, steps):
    if direction == 'left':
        for i in range(steps):
            string.append(string.pop(0))
    if direction == 'right':
        for i in range(steps):
            string.insert(0, string.pop())

       
def rotate_letter(string, letter):
    steps = string.index(letter)
    if steps >= 4:
        steps += 1
    steps += 1
    rotate(string, 'right', steps)


def move_position(string, pos1, pos2):
    letter = string.pop(pos1)
    string.insert(pos2, letter)

  
def reverse(string, fr, to):
    if fr != 0:
        r = string[to:fr - 1:-1]
    else:
        r = string[to::-1]
    return string[:fr] + r + string[to + 1:]


def h(string):
    string = list(string)
    for line in instructions:

        if line[0] == 'swap':
            if line[1] == 'position':
                swap_position(string, int(line[2]), int(line[-1]))
            else:
                swap_letter(string, line[2], line[-1])


        if line[0] == 'rotate':
            if line[1] == 'based':
                rotate_letter(string, line[-1])
            else:
                rotate(string, line[1], int(line[-2]))


        if line[0] == 'move':
            move_position(string, int(line[2]), int(line[-1]))


        if line[0] == 'reverse':
            string = reverse(string, int(line[2]), int(line[-1]))

    return ''.join(string)


perms = [''.join(p) for p in permutations('fbgdceah')]

for pwd in perms:
    if h(pwd) == 'fbgdceah':
        print pwd
        break
        