import matplotlib.pyplot as plt


def pi_raw_python(n: int) -> float:
    pi = 0
    for k in range(n):
        pi += 2 / ((4 * k + 1) * (4 * k + 3))
    return pi * 4


def evaluator(n: int, func) -> tuple[list[float], list[float]]:
    import time
    times = []
    values = []
    for i in range(n):
        start = time.time()
        func(i)
        end = time.time()
        times.append(end - start)
        values.append(func(i))
    return times, values


def comparator(n: int, func1, func2):
    times1, values1 = evaluator(n, func1)
    times2, values2 = evaluator(n, func2)
    func1_name = func1.__name__
    func2_name = func2.__name__
    ns = list(range(n))
    plt.figure(figsize=(12, 6))
    
    plt.subplot(1, 2, 1)
    plt.plot(ns, times1, label=func1_name)
    plt.plot(ns, times2, label=func2_name)
    plt.title('Execution Time')
    plt.legend()
    
    plt.subplot(1, 2, 2)
    plt.plot(ns, values1, label=func1_name)
    plt.plot(ns, values2, label=func2_name)
    plt.title('Function Values')
    plt.legend()
    
    plt.show()


if __name__ == '__main__':
    from digits_pi import calculate_pi

    comparator(10000, pi_raw_python, calculate_pi)
