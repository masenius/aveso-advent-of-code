#!/usr/bin/env python

f = open("d3.input", 'r')
number_of_triangles = 0


def is_triangle(triangle):
    triangle.sort()
    print triangle
    return (triangle[0] + triangle[1] > triangle[2])


for triangle in f.readlines():
    triangle = [int(x) for x in triangle.strip().split()]
    number_of_triangles += is_triangle(triangle)

print number_of_triangles
