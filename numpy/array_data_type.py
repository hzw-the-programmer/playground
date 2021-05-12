import numpy as np

a = np.array([1, 2])
print(a.dtype)

a = np.array([1., 2.])
print(a.dtype)

a = np.array([1, 2], dtype = np.float64)
print(a.dtype)

a = np.array([1., 2.], dtype = np.int64)
print(a.dtype)
