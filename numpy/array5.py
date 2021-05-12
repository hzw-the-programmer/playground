import numpy as np

a = np.array([(1, 2), (3, 4)])
#b = np.array([(5, 6, 7), (8, 9, 10)])
#b = np.array([(5, 6, 7), (8, 9, 10), (11, 12, 13)])
#print(np.hstack((a, b)))
b = np.array([(5, 6)])
#b = np.array([(5, 6, 7)])
print(np.vstack((a, b)))
