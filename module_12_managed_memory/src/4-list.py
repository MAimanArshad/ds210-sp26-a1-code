v = [[1, 2], [3, 4]]
x = v[0]


print(hex(id(v[0])))
print(hex(id(x)))

x.append(50)
print(v)

for i in range(5):
    v.append(i)

print(v)

print(x)
print(hex(id(v[0])))
print(hex(id(x)))
