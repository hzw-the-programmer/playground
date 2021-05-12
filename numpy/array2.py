import numpy as np

a = np.arange(64)
a = a.reshape(4, 4, 4)

print(a)
print()

print(a[1])
print()

print(a[1, 1])
print()

print(a[1, 1, 1])
print()

b = a[1:3]
print(b)
print(b.min())
print(b.max())
print(b.sum())
print(b.sum(axis = 0))
print(b.sum(axis = 1))
print(b.sum(axis = 2))
print()

b = a[1:3, 1:3]
print(b)
print(b.min())
print(b.max())
print(b.sum())
print(b.sum(axis = 0))
print(b.sum(axis = 1))
print(b.sum(axis = 2))
print()

b = a[1:3, 1:3, 1:3]
print(b)
print(b.min())
print(b.max())
print(b.sum())
print(b.sum(axis = 0))
print(b.sum(axis = 1))
print(b.sum(axis = 2))
print()
