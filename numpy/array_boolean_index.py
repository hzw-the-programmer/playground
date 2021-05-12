import numpy as np

a = np.array([[1, 2], [3, 4], [5, 6]])
b = a > 2
print(b)

print('************')

c = a[b]
print(c)
c[0] = 99
print(c)
print(a)

print('************')

a[b] += 10
print(a)
