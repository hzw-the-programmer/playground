import numpy as np

x = np.array([[1, 2], [3, 4]])

print(x.mean())
print(x.var())
print(x.std())

print(x.mean(axis=0))
print(x.var(axis=0))
print(x.std(axis=0))

print(x.mean(axis=1))
print(x.var(axis=1))
print(x.std(axis=1))
