#!/usr/bin/env python


def reversed(number, num_bits):
    answer = 0
    for i in range(num_bits):
        if number & (1 << i):
            answer |= 1 << (num_bits - 1 - i)
    return answer


def double_it(number, mask):
    return reversed(number, mask.bit_length()) ^ mask | number << (mask.bit_length() + 1), 2 ** (mask.bit_length() * 2 + 1) - 1


def checksum(x, num_bits):
    answer = 0
    t = num_bits
    while t > 0:
        answer = answer << 1 | 1 - ((x & 1) ^ ((x >> 1) & 1))
        x >>= 2
        t -= 2
    num_bits = num_bits / 2
    answer = reversed(answer, num_bits)
    if num_bits % 2 != 0:
        return answer, num_bits
    return checksum(answer, num_bits)


number = 0b01000100010010111
mask = 0b11111111111111111
disk_length = 272

while mask.bit_length() < disk_length:
    number, mask = double_it(number, mask)

number = number >> (mask.bit_length() - disk_length)
mask = 2 ** disk_length - 1

number, length = checksum(number, mask.bit_length())
print bin(number), length
