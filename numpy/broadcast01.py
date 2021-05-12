import numpy as np

a = np.array([[1, 2, 3], [4, 5, 6], [7, 8, 9], [10, 11, 12]])
b = np.array([1, 0, 1])
c = np.empty_like(a)
print(c)

for i in range(4):
    c[i, :] = a[i, :] + b

print(c)

print('******')

bb = np.tile(b, (4, 1))
print(bb)
c = a + b
print(c)

print('******')

c = a + b
print(c)
