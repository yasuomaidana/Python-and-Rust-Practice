def nested_sum(iterations, f):
    result = 0
    for i in range(iterations):
        result += f(i,result)
    return result