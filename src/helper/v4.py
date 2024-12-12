import math
from v3 import plot_function_over_range

f0 = lambda x: 0
f1 = lambda x: 1 + x ** 2
f2 = lambda x: 1 + x + x ** 2 - 3 * x ** 3 + x ** 4
f3 = lambda x: math.exp(x) / (1 + math.exp(x)) + 0.001 * x  # the linear term is needed because of prescision errors
f4 = lambda x: math.sin(x)
f5 = lambda x: x if x <= 0 else 1 + 0.2*x if x <= 9 else 3
f6 = lambda x: 2 * x
f7 = lambda x: 5
f8 = lambda x: 1 / x if x != 0 else 1
f9 = lambda x: math.sin(1 / x) if x != 0 else 1

tests = [f0, f1, f2, f3, f4, f5, f6, f7, f8, f9]

# Pads a range (low, high) by some percantance of it's width
def pad_range(rge, padding):
    return [rge[0] - (rge[1] - rge[0]) * padding, rge[1] + (rge[1] - rge[0]) * padding]

# Takes the derivative of a function by the central difference method
def der(f, d=1e-6):
    return lambda x: (f(x + d) - f(x - d)) / (2 * d)

# Performs iterative gradient descent on a function
def grad_desc(f, start, steps=1000, temp=1e-8):
    curr_x = start
    curr_f = f(start)

    for _i in range(steps):
        if (dfx := der(f)(curr_x)) == 0:
            break

        prev_x, prev_f = curr_x, curr_f

        curr_x -= temp * dfx

        if prev_f >= curr_f:
            temp /= 10
            curr_x = prev_x
        
        else:
            temp *= 2
    
    return curr_x

def grad_desc_to_zero(f, start, steps=1000):
    return grad_desc(lambda x: f(x) ** 2, start)

def grad_desc_to_stat(f, start, steps=1000):
    return grad_desc_to_zero(der(f), start, steps)

# Subdivides the interval (low, high) into n equal parts (len(result) == n + 1)
def subdivide(low, high, n):
    return [low + (high - low) * i / n for i in range(n + 1)]

# Removes duplicates, within some error value
def distinct_floats(list, epsilon=0.001):
    if len(list) <= 1 or not epsilon:
        return list
    
    prev = float('inf')
    o = []
    for i in sorted(list):
        if abs(i - prev) > epsilon:
            o.append(i)
        prev = i
    
    return o

# Returns the n smallest magnitude values of li, or all of them if not flag
def sorted_least(li, n=5, flag=True, key=abs):
    return list(sorted(li, key=lambda x: key(x)))[:min(len(li), n) if flag else len(li)]

def stationary_points(f, low=-100, high=100, cuts=1000, extend_below=True, extend_above=True, depth=20, max_point_count=50, cuts_divisor=5):
    if cuts <= 1:
        return (False, [(low + high) / 2])
    
    x = subdivide(low, high, cuts)
    y = [f(p) for p in x]

    dy = [(y[i + 1] - y[i]) / (x[i + 1] - x[i]) for i in range(len(x) - 1)]

    is_inf = False
    o = []

    for i in sorted(range(len(dy) - 1), key=lambda i: abs(x[i])):
        if dy[i] * dy[i + 1] <= 0:
            sp = stationary_points(f, x[i], x[i + 2], cuts // cuts_divisor, False, False, depth - 1, max_point_count, cuts_divisor)
            o += sp[1]

            if sp[0]:
                is_inf = True
                break
            
            if len(o) > max_point_count:
                break
    
    if cuts == 1000:
        pass

    # Choose distinct points at second-to-last depth
    if cuts // cuts_divisor <= 1:  # Only at second-to-last depth
        epsilon = (high - low) / (cuts - 1)  # Slightly larger than the inter-point interval
        o = distinct_floats(o, epsilon)
    
    # Conditions indicating infinite stationary points
    if depth <= 0 or len(o) >= max_point_count or is_inf:
        is_inf = True
        return (is_inf, [grad_desc_to_stat(f, x, 100) for x in sorted_least(o, flag=is_inf)])
    
    # Extensions above and below
    if o and extend_below:
        sp = stationary_points(f, low - (high - low), low, cuts // cuts_divisor, True, False, depth - 1, max_point_count, cuts_divisor)
        is_inf = is_inf or sp[0]
        o += sp[1]
    
    if o and extend_above:
        sp = stationary_points(f, high, high + (high - low), cuts // cuts_divisor, False, True, depth - 1, max_point_count, cuts_divisor)
        is_inf = is_inf or sp[0]
        o += sp[1]
    
    return (is_inf, [grad_desc_to_stat(f, x, 100) for x in sorted_least(o, flag=is_inf)])

def stat_points(f):
    is_inf, points = stationary_points(f)
    
    epsilon = (max(points) - min(points)) / len(points) if points else 0
    points = distinct_floats(points, epsilon)

    # Bounds check
    return (is_inf, [p for p in points if abs(p) <= 1e18])

def ntor(nums, pad=0.5):  # nums to range
    return pad_range((min(nums), max(nums)), pad)

def zeros(f):
    zero_points = [grad_desc_to_zero(f, z) for z in sorted(f(x) ** 2 for x in subdivide(-100, 100, 201))[:10]]
    threshold = 10 * min(f(z) for z in zero_points)
    return [x for x in zero_points if f(x) <= threshold]  # A .filter() would be better

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

    p = distinct_floats(p, 1e-4)

    if len(p) <= 1:
        p += stat_points(lambda x: (abs(der(f)(x)) - 1) ** 2)[1]

    if len(p) <= 1:
        p += zeros(f)

    if len(p) <= 1:
        return (-10, 10)

    return ntor(p)

print('test', determine_range(f4))

for fi, f in enumerate(tests):
    print(fi, determine_range(f))

for t in tests:
    r = determine_range(t)
    print(r)
    plot_function_over_range(t, r)









    