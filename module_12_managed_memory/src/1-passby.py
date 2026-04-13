# Is l1 passed to l2 by move, clone/copy, or ref?
def helper(l2):
    l2.append([100, 200])
    print(l2)

l1 = [[1, 2], [10, 20]]
helper(l1)
print(l1)
