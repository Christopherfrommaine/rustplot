# Pads a range (low, high) by some percantance of it's width
def pad_range(rge, padding):
    return [rge[0] - (rge[1] - rge[0]) * padding, rge[1] + (rge[1] - rge[0]) * padding]

# Takes the derivative of a function by the central difference method
def der(f, d=1e-6):
    return lambda x: (f(x + d) - f(x - d)) / (2 * d)

# Performs iterative gradient descent on a function
def grad_desc(f, start, steps=100, temp=1e-8):
    curr_x = start
    curr_f = f(start)

    for _i in range(steps):
        if (dfx := der(f)(curr_x)) == 0:
            break

        prev_x, prev_f = curr_x, curr_f

        curr_x -= temp * dfx

        if prev_f >= curr_f:
            temp /= 2
            curr_x = prev_x
        else:
            temp *= 2
    
    return curr_x

# Finds local zeros of a function
def grad_desc_to_zero(f, start, steps=100):
    return grad_desc(lambda x: f(x) ** 2, start, steps)

# Finds local zeros of a fucntion's derivative
def grad_desc_to_stat(f, start, steps=100):
    return grad_desc_to_zero(lambda x: der(f)(x) ** 2, start, steps)

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
def sorted_least(li, n=5, flag=True, key=lambda x: x ** 2):
    return sorted(li, key=lambda x: key(x))[:min(len(li), n) if flag else len(li)]

# Internal function for use of finding stationary points of a function with extensions and recursive accuracy improvement
def stationary_points(f, low=-100, high=100, cuts=1000, extend_below=True, extend_above=True, depth=20, max_point_count=50, cuts_divisor=5):
    # Recursive base case
    if cuts <= 1:
        return (False, [(low + high) / 2])
    
    # Range of x and y values to search
    x = subdivide(low, high, cuts)
    y = [f(p) for p in x]

    # Slopes of y at each x
    dy = [(y[i + 1] - y[i]) / (x[i + 1] - x[i]) for i in range(len(x) - 1)]

    # Return values: (
    # are there an infinite number of stationary points?,
    # stationary points or first n stationary points if is_inf)
    is_inf = False
    o = []

    # Start at numbers closer to zero. Theres probably a much faster way to do this than sort (i.e. O(n) rather than O(nlog(n)))
    for i in sorted(range(len(dy) - 1), key=lambda i: x[i] ** 2):
        # If the derivative changes sign (e.g., goes from + to -. There must be a f'(x) == 0 in between, or a cusp)
        if dy[i] * dy[i + 1] <= 0:
            # Recursively search the range at a higher detail, to improve prescision and find distinct close-together stationary points
            sp = stationary_points(f, x[i], x[i + 2], cuts // cuts_divisor, False, False, depth - 1, max_point_count, cuts_divisor)
            
            # Add points to output and check for infinity
            o += sp[1]
            if sp[0] or len(o) > max_point_count:
                is_inf = True
                break

    # Choose distinct points at second-to-last depth
    if cuts // cuts_divisor <= 1:  # Only at second-to-last depth
        epsilon = (high - low) / (cuts - 1)  # Slightly larger than the inter-point interval
        o = distinct_floats(o, epsilon)
    
    # Conditions indicating infinite stationary points
    if depth <= 0 or len(o) >= max_point_count or is_inf:
        is_inf = True
        return (is_inf, [grad_desc_to_stat(f, x) for x in sorted_least(o, flag=is_inf)])
    
    # Extend above if still finding new points
    if o and extend_below:
        sp = stationary_points(f, low - (high - low), low, cuts // cuts_divisor, True, False, depth - 1, max_point_count, cuts_divisor)
        is_inf = is_inf or sp[0]
        o += sp[1]
    
    # Extend below if still finding new points
    if o and extend_above:
        sp = stationary_points(f, high, high + (high - low), cuts // cuts_divisor, False, True, depth - 1, max_point_count, cuts_divisor)
        is_inf = is_inf or sp[0]
        o += sp[1]
    
    # return a grad desc the points (or a small number of points if there are an infinite number)
    return (is_inf, [grad_desc_to_stat(f, x) for x in sorted_least(o, flag=is_inf)])

# Mostly a wrapper for stationary_points() with a couple improvements to it's output
def stat_points(f):
    is_inf, points = stationary_points(f)
    
    # Group together nearby (likely equal) points
    epsilon = (max(points) - min(points)) / len(points) if points else 0
    points = distinct_floats(points, epsilon)

    # Bounds check (within 10^18)
    return (is_inf, [p for p in points if abs(p) <= 1e18])

# Given a list of nums, return the range they occupy, plus some pading. ntor = nums to range
def ntor(nums, pad=0.5):
    return pad_range((min(nums), max(nums)), pad)

# Finds zeros of a function using gradient descent
def zeros(f):
    # Much coarser than stat_points(). Just looks for approximate solutions
    zero_points = [grad_desc_to_zero(f, z) for z in sorted(f(x) ** 2 for x in subdivide(-100, 100, 201))[:10]]
    threshold = 10 * min(f(z) for z in zero_points)
    return [x for x in zero_points if f(x) <= threshold]  # A .filter() would be better

# Checks if a function f(x) == 0 for all x
def is_only_zero(f):
    return all(abs(f(x)) <= 0.001 for x in subdivide(-100, 100, 1001))

# Main function to determine the plot range of a given function
def determine_range(f):
    if is_only_zero(der(f)):
        # constant
        return ntor([-1, 1])

    if is_only_zero(der(der(f))):
        # affine
        return ntor([0, 1] + zeros(f))

    # Finds a bunch of 'important' points (p) to a function. Makes a range which displays them all.
    p = []
    p += stat_points(f)[1]  # Stationary Points
    p += stat_points(der(f))[1]  # Change-of-curvature points

    # If there may be a too-small range over which points are checked
    if len(distinct_floats(p, 0.1)) <= 1:
        p += stat_points(lambda x: (abs(der(f)(x)) - 1) ** 2)[1]  # f'(x) == 1 points

    p = distinct_floats(p, 1e-4)  # Consolidate points

    if len(p) <= 1:
        # If still not enough points, 
        p += zeros(f)  # f(x) == 0 points

    # Backup case, if everything else doens't work
    if len(p) == 1:
        return (p[0] - 10, p[0] + 10)
    if len(p) == 0:
        return (-10, 10)

    return ntor(p)


# Testing
if __name__ == "__main__":
    import math

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

    # Tests
    for fi, f in enumerate(tests):
        print(fi, determine_range(f))

    for t in tests:
        r = determine_range(t)
        print(r)
        plot_function_over_range(t, r)
