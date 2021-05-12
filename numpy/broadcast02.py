import numpy as np

v = np.array([1, 2, 3]) # (3,)
w = np.array([4, 5]) # (2,)

#print(np.reshape(v, (3, 1)))
print(np.reshape(v, (3, 1)) * w)

print('*' * 18)

x = np.array([[1, 2, 3], [4, 5, 6]]) # (2, 3)
print(x + v)

print('*' * 18)

print((x.T + w).T)

print('*' * 18)

print(x + np.reshape(w, (2, 1)))

print(x * 2) # () -> (1, 1)

# Broadcasting two arrays together follows these rules:

# 1. If the arrays do not have the same rank,
#    prepend the shape of the lower rank array with 1s until both shapes have the same length.
# 2. The two arrays are said to be compatible in a dimension if they have the same size in the dimension,
#    or if one of the arrays has size 1 in that dimension.
# 3. The arrays can be broadcast together if they are compatible in all dimensions.
# 4. After broadcasting, each array behaves as if it had shape equal to
#    the elementwise maximum of shapes of the two input arrays.
# 5. In any dimension where one array had size 1 and the other array had size greater than 1,
#    the first array behaves as if it were copied along that dimension
