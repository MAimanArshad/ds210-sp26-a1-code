import sys

x = [1, 2, 3]
y = x
x.append(4)
y.append(5)
print(x)
print(y)
















print(hex(id(x)))
print(hex(id(y)))
























print(sys.getrefcount(x))

del y
print(sys.getrefcount(x))
