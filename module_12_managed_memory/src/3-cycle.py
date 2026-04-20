import sys
import gc

x = [1, 2, 3]
x.append(x)
x.append(x)


print(hex(id(x)))
print(sys.getrefcount(x))














#del x

#garbage_items = gc.collect()
#print(garbage_items)
