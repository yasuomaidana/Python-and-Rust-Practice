from to_call.nested_sum import nested_sum
from to_call.single_function import simple_sum


def complex_sum(n):
    return nested_sum(n, simple_sum)
