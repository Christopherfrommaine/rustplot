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
f3 = lambda x: math.exp(x) / (1 + math.exp(x)) + 0.001 * x  # the linear term is needed because of prescision errors
f4 = lambda x: math.sin(x)
f5 = lambda x: x if x <= 0 else 1 + 0.2*x if x <= 9 else 3
f6 = lambda x: 2 * x
f7 = lambda x: 5
f8 = lambda x: 1 / x if x != 0 else 1
f9 = lambda x: math.sin(1 / x) if x != 0 else 1

tests = [f1, f2, f3, f4, f5, f6, f7, f8, f9]

import math

f10 = lambda x: math.sqrt(x) if x >= 0 else -1  # Square root, undefined for negative x
f11 = lambda x: math.log(x) if x > 0 else float('-inf')  # Logarithm, undefined for non-positive x
f12 = lambda x: x**3 - 4 * x**2 + 6 * x - 24  # Cubic polynomial
f13 = lambda x: math.cos(x) + math.sin(x)  # Trigonometric sum
f14 = lambda x: math.tanh(x)  # Hyperbolic tangent
f15 = lambda x: abs(x)  # Absolute value
f16 = lambda x: x % 2  # Modulo operation
f17 = lambda x: 0 if x == 0 else x / abs(x)  # Sign function
f18 = lambda x: (x**2 + 1) / (x + 1) if x != -1 else float('inf')  # Rational function with singularity
f19 = lambda x: math.floor(x)  # Floor function
f20 = lambda x: math.factorial(int(x)) if x >= 0 and x == int(x) else None  # Factorial for integers
f21 = lambda x: math.gamma(x)  # Gamma function (generalized factorial)
f22 = lambda x: x * math.exp(-x)  # Exponentially decaying function

moretests = tests + ([f10, f11, f12, f13])


# Pads a range (low, high) by some percantance of it's width
def pad_range(rge, padding):
    return [rge[0] - (rge[1] - rge[0]) * padding, rge[1] + (rge[1] - rge[0]) * padding]

# Takes the derivative of a function
def der(f, d=0.000001):
    return lambda x: (f(x + d) - f(x)) / d

# Performs iterative gradient descent on a function
def grad_desc(f, start, steps=1000, temp=1):
    x = start
    for i in range(steps):
        if (dfx := der(f)(x)) == 0:
            break
        x -= temp * dfx * (steps / (2 * i + steps))
    return x

# Improved gradient descent function, which only allows for downward motion and does not need a set temp
def grad_desc_towards_zero(f, start, steps=1000):
    temp = 1
    curr_f = f(start)

    # return start

    x = start
    for i in range(steps):
        if (dfx := der(f, 0.0001 * temp)(x)) == 0:
            break
        prev_x = x
        prev_f = curr_f

        if abs(temp * dfx) >= abs(1e3 * x):
            temp /= 100
            return start
        else:

            x -= temp * dfx
            curr_f = f(x)

            if curr_f > prev_f:
                temp /= 5
                x = prev_x
                curr_f = prev_f
    return x

# Uses gradient descent to find a stationary point of a function (i.e. f'(x) == 0)
def grad_desc_to_stationary_point(f, start, steps):
    return grad_desc_towards_zero(lambda x: der(f)(x) ** 2, start, steps)

# Subdivides the interval (low, high) into n - 1 pieces
def subdivide(low, high, n):
    return [low + (high - low) * i / (n - 1) for i in range(n + 1)]

# Removes duplicates, within some error value
def distinct_floats(list, epsilon=0.001):
    if len(list) <= 1 or not epsilon:
        return list
    o = []
    checked = []
    for i in list:
        if not any(abs(i - e) <= epsilon for e in checked):
            o.append(i)
        checked.append(i)
    return o

# Returns the num smallest values of li, or all of them if not flag
def sorted_least(li, num=5, flag=True):
    return list(sorted(li, key=lambda x: abs(x)))[:min(len(li), num) if flag else len(li)]

# Finds all stationary points of a function, within (low, high) starting with cuts checks
def stationary_points(f, low=-100, high=100, cuts=1001, extend_above=True, extend_below=True, extend_depth=8, max_o_count=100) -> tuple[bool, list]:
    if cuts <= 1:
        return (False, [(high + low) / 2])

    o = []
    is_inf = False

    cuts_divisor = 5

    p = subdivide(low, high, cuts)
    y = [f(x) for x in p]

    dx = p[:-1] 
    dy = [(y[i + 1] - y[i]) / (p[i + 1] - p[i]) for i in range(len(p) - 1)]

    for i in sorted(range(len(dx) - 2), key=lambda x: abs(dx[x])):
        if dy[i] * dy[i + 1] <= 0:
            # Interior Recursion; Narrowing on the specific point of sign change
            sp = stationary_points(f, dx[i], dx[i + 2], cuts // cuts_divisor, False, False, extend_depth - 1)
            o += sp[1]

            if sp[0]:
                is_inf = True
                break
        
        if len(o) > max_o_count:
            break
    
    if extend_depth == 20:
        pass

    # Slightly larger than inter-point intervals to capture flat lines
    epsilon = (high - low) / (cuts - 2) if cuts > 2 else 1
    if cuts // cuts_divisor > 1:
        epsilon = 0
    
    if extend_depth >= 0 and len(o) <= max_o_count and not is_inf:
        if o:
            if extend_below:
                sp_lower = stationary_points(f, low - (high - low) * 1, low, cuts, False, True, extend_depth - 1, max_o_count - len(o))
                is_inf = is_inf or sp_lower[0]
                o += sp_lower[1]
            if extend_above:
                sp_upper = stationary_points(f, high, high + (high - low) * 1, cuts, True, False, extend_depth - 1, max_o_count - len(o))
                is_inf = is_inf or sp_upper[0]
                o += sp_upper[1]
            return (is_inf, distinct_floats([grad_desc_to_stationary_point(f, x, 100) for x in sorted_least(o, flag=is_inf)], epsilon))
        
        else:
            return (False, [])
    else:
        return (True, distinct_floats([grad_desc_to_stationary_point(f, x, 100) for x in sorted_least(o)], epsilon))

# Clean up and make the data from stationary points more accurate
def stat_points(f, low=-100, high=100, cuts=1001, max_depth=8, max_points=100, epsilon_sensitivity=0.00001):
    is_inf, points = stationary_points(f, low, high, cuts, extend_depth=max_depth, max_o_count=max_points)
    
    closer_points = [grad_desc_to_stationary_point(f, p, 10000 // len(points)) for p in points]

    function_range = abs(f(high)) + abs(f(low)) + abs(f((high + low) / 2))
    
    epsilon = epsilon_sensitivity * math.log2(abs(function_range)) if function_range else 1

    closer_points = distinct_floats(closer_points, epsilon)

    within_bounds = [p for p in closer_points if abs(p) <= 1e18]

    return (is_inf, within_bounds)

# Test
# for f in tests:
#     print(stat_points(f))


def ntor(nums, pad=0.5):  # nums to range
    return pad_range((min(nums), max(nums)), pad)


import numpy as np
from matplotlib import pyplot as plt

def plot_function_over_range(f, range=(-10, 10), num_points=1000):
    x = np.array(subdivide(range[0], range[1], num_points))
    y = np.array([f(p) for p in x])
    
    plt.figure(figsize=(8, 6))
    plt.plot(x, y)
    plt.axhline(0, color='black', linewidth=0.5, linestyle='--')
    plt.axvline(0, color='black', linewidth=0.5, linestyle='--')
    plt.grid(color='gray', linestyle='--', linewidth=0.5)
    plt.title('Function Plot')
    plt.xlabel('x')
    plt.ylabel('f(x)')

    y_mean = sum(y) / len(y)
    y_std_dev = sum((z - y_mean) ** 2 for z in y) / len(y)
    y_range = (y_mean - 3 * y_std_dev, y_mean + 3 * y_std_dev)

    if y_std_dev <= 0.000000001:
        y_range = (y_mean - 1, y_mean + 1)
    ys = list(sorted(y, key=lambda x: abs(x)))[:-5]

    y_range = (min(ys), max(ys))
    if y_range[1] - y_range[0] <= 0.000000001:
        y_range = (y_range[0] - 1, y_range[1] + 1)
    y_range = pad_range(y_range, 0.1)
    
    plt.ylim(y_range)

    plt.show()


def zeros(f):
    zero_points = [grad_desc_towards_zero(lambda x: f(x) ** 2, z, 100) for z in list(sorted(f(x) ** 2 for x in subdivide(-100, 100, 201)))[:10]]
    return [x for x in zero_points if f(x) <= 0.001]

def is_only_zero(f):
    return all(abs(f(x)) <= 0.001 for x in subdivide(-100, 100, 1001))

def determine_range(f):
    if is_only_zero(der(f)):
        # constant
        return ntor([-1, 1])

    if is_only_zero(der(der(f))):
        # affine
        return ntor([0, 1] + zeros(f))

    p = []
    p += stat_points(f)[1]
    p += stat_points(der(f))[1]

    if len(p) <= 1:

        p += stat_points(lambda x: (abs(der(f)(x)) - 1) ** 2)[1]

        if len(p) <= 1:

            p += zeros(f)

    return ntor(p)


if __name__ == "__main__":
    for fi, f in enumerate(tests):
        print(fi, determine_range(f))

    for t in tests:
        r = determine_range(t)
        print(r)
        plot_function_over_range(t, r)