import numpy as np

a = np.array([[1, 2, 3], [4, 5, 6], [7, 8, 9], [10, 11, 12]])
b = [0, 2, 0, 1]

c = a[np.arange(4), b]
print(c)
c[0] = 99
print(c)
print(a)

print('************')

a[np.arange(4), b] += 10
print(a)
