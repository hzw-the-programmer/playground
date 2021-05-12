import numpy as np

a = np.array([[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]])
b = a[:2, 1:3]
print(b)
b[0, 0] = 99
print(b)
print(a)

print('************')

b = a[2, 1:3]
print(b)
b[0] = 99
print(b)
print(a)

print('************')
