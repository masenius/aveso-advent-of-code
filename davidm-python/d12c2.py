#!/usr/bin/env python

f = open("d12.input", "r")

instructions = []

for line in f.readlines():
    line = line.split()
    operator = line[0]
    if 'dec' in operator or 'inc' in operator:
        address = ord(line[1]) - 97
        instructions.append([operator, address])
    if 'cpy' in operator or 'jnz' in operator:
        fr = line[1]
        to = line[2]
        instructions.append([operator, fr, to])

registers = [0, 0, 1, 0]

instruction = 0
while instruction < len(instructions):
    operator = instructions[instruction][0]

    if 'cpy' in operator:
        fr = instructions[instruction][1]
        to = ord(instructions[instruction][2]) - 97
        if fr.isdigit():
            registers[to] = int(fr)
        else:
            registers[to] = registers[ord(fr) - 97]
    if 'inc' in operator:
        address = instructions[instruction][1]
        registers[address] += 1
    if 'dec' in operator:
        address = instructions[instruction][1]
        registers[address] -= 1
    if 'jnz' in operator:
        fr = instructions[instruction][1]
        steps = int(instructions[instruction][2])
        if fr.isdigit() and int(fr) != 0 or registers[ord(fr) - 97] != 0:
            instruction += steps - 1

    instruction += 1

print registers
