import numpy as np

x = np.array([[1, 2], [3, 4]])

print(x.min())
print(x.min(axis=0))
print(x.min(axis=1))

print(x.max())
print(x.max(axis=0))
print(x.max(axis=1))
