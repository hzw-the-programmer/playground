import numpy as np

a = np.array([(1, 4, 9), (16, 25, 36)])
print(np.sqrt(a))
print(np.std(a))

b = np.arange(6)
b = b.reshape(2, 3)

print()
print(a)
print()
print(b)
print()
print(a + b)
print()
print(a - b)
print()
print(a * b)
print()
print(b / a)
print()
