import math
import numpy as np

f1 = lambda x: 1 + x ** 2
f2 = lambda x: 1 + x + x ** 2 - 3 * x ** 3 + x ** 4
f3 = lambda x: math.exp(x) / (1 + math.exp(x))
f4 = lambda x: math.sin(x)
f5 = lambda x: x if x <= 0 else 1 + 0.2*x if x <= 9 else 3

tests = [f1, f2, f3, f4, f5]

def der(f, x, d=0.001):
    return (f(x + d) - f(x)) / d

def grad_desc(f, start, steps=1000, temp=0.001):
    x = start;
    for _ in range(steps):
        x -= temp * der(f, x)
    return x

def transform_to_minimizable(f):
    return lambda x: f(x) ** 2

def distinct_floats(list, epsilon=0.01):
    o = []
    for i in list:
        if not any(abs(i - e) <= epsilon for e in o):
            o.append(i)
    return o

def range_grid_desc(f, low, high, cuts):
    return distinct_floats(grad_desc(f, float(x)) for x in np.linspace(low, high, cuts))

def zeros_in_range(f, low, high, cuts):
    return range_grid_desc(transform_to_minimizable(f), low, high, cuts)

def tending_towards_infinity(list, neg_inf, pos_inf):
    left = False
    right = False
    for x in list:
        if x >= pos_inf:
            right = True
        if x <= neg_inf:
            left = True
    return (left, right)

def all_zeros(f):
    r = [-10, 10]

    for _ in range(20):
        dr = r[1] - r[0]
        mr = (r[0] + r[1]) / 2

        zeros = zeros_in_range(f, r[0], r[1], 100)
        print(zeros)
        if not zeros:
            r[0] -= dr
            r[1] += dr
        else:
            li, ri = tending_towards_infinity(zeros, mr - 0.5 * dr, mr + 0.5 * dr)
            if li:
                # if low numbers are tending towards -inf (i.e. horizontal asymtote)
                r[0] = min(zeros)
            if ri:
                r[1] = max(zeros)
    
    return zeros

print(all_zeros(f1))
        


