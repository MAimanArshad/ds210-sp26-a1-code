def large_list(n):
    l = []
    for i in range(0, n):
        l.append((i % 9) - 4)
    return l