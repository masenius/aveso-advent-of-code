#!/usr/bin/env python

f = open("d3.input", 'r')
number_of_triangles = 0
sides = [[], [], []]
triangles = []


def is_triangle(triangle):
    triangle.sort()
    return (triangle[0] + triangle[1] > triangle[2])


for row in f.readlines():
    row = [int(x) for x in row.strip().split()]
    for i in range(0, 3):
        sides[i].append(row[i])

sides = sides[0] + sides[1] + sides[2]

triangle = []
for side in sides:
    triangle.append(side)
    if len(triangle) == 3:
        triangles.append(triangle)
        triangle = []

for triangle in triangles:
    number_of_triangles += is_triangle(triangle)

print number_of_triangles
