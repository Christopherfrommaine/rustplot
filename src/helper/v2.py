import math

"""
Given a function f
Find all stationary points
if len(stat_points) == infinity:
    choose 5 closest to x=0
if len(stat_points) == 1:
    f_prime_range = this for f'
    if f_prime_range = None:
        # No real good way to do this
        x_range = stat_points[0] +/- 1
    else:
        x_range = (min(stat_points[0], f_prime_range[0]), max(stat_points[0], f_prime_range[1]))
elif len(stat_points) != 0:
    x_range = pad(min(stat_points), max(stat_points))
else:
    # no stat points
    if do_second_der_test:
        x_range = this for f'
    else:
        find all zeros of f
        if len(zeros_f) == 0:
            x_range = (-10, 10)
        elif len == 1:
            x_range = zeros_f + (-10, 10)
        elif len == infinty:
            x_range = 5 closest zeros to x=0
        else:
            x_range = min(zeros), max(zeros) + padding
        

"""


f1 = lambda x: 1 + x ** 2
f2 = lambda x: 1 + x + x ** 2 - 3 * x ** 3 + x ** 4
f3 = lambda x: math.exp(x) / (1 + math.exp(x))
f4 = lambda x: math.sin(x)
f5 = lambda x: x if x <= 0 else 1 + 0.2*x if x <= 9 else 3
f6 = lambda x: 5

tests = [f1, f2, f3, f4, f5, f6]

def pad_range(rge, padding):
    return [rge[0] - (rge[1] - rge[0]) * padding, rge[1] + (rge[1] - rge[0]) * padding]

def der(f, d=0.0001):
    return lambda x: (f(x + d) - f(x)) / d

def grad_desc(f, start, steps=1000, temp=1):
    x = start
    for i in range(steps):
        if (dfx := der(f)(x)) == 0:
            break
        x -= temp * dfx * (steps / (2 * i + steps))
    return x

def subdivide(low, high, n):
    return [low + (high - low) * i / (n - 1) for i in range(n + 1)]

def distinct_floats(list, epsilon=0.001):
    if len(list) <= 1:
        return list
    o = []
    for i in list:
        if not any(abs(i - e) <= epsilon for e in o):
            o.append(i)
    return o

def find_zeros(f, low, high, cuts: 100, extend=True, infinity=100000, max_steps=100):
    if cuts <= 2:
        return (False, [(low + high) / 2])
    o = []
    p = subdivide(low, high, cuts)
    y = [f(x) for x in p]

    for steps in range(max_steps):
        added_some = False

        # Uniform Testing
        for i in range(len(p) - 1):
            # Multiply = check for opposite signs
            if y[i] * y[i+1] <= 0:
                added_some = True
                o += find_zeros(f, p[i], p[i+1], cuts // 3, extend=False)[1]
        
        if o and extend and added_some:
            padded_range = pad_range([min(o), max(o)], 1)  # pads to 200%
            p = (subdivide(padded_range[0], low, cuts) if padded_range[0] < low else []) +\
                (subdivide(high, padded_range[1], cuts) if padded_range[1] > high else [])
            y = [f(x) for x in p]
            if not p:
                return (False, o)
        else:
            break

        if max(p) > infinity or min(p) < -infinity:
            # has infinite zeros
            return (True, list(sorted(o[:min(5, len(o))], key=lambda x: abs(x))))

    # Optimization
    epsilon = 0.1 * (max(p) - min(p)) / cuts
    return (False, distinct_floats([grad_desc(lambda x: f(x) ** 2, x, 10, epsilon) for x in o], 100 * epsilon))

def stationary_points(f):
    high, low, cuts = -10, 10, 101
    df = der(f, 0.0001 * (high - low) / cuts)
    return find_zeros(df, high, low, cuts)

for f in tests:
    print(stationary_points(f))