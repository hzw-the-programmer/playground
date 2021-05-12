import numpy as np
import matplotlib.mlab as mlab
import matplotlib.pyplot as plt

# x = [21,22,23,4,5,6,77,8,9,10,31,32,33,34,35,36,37,18,49,50,100]
# num_bins = 5
x = [1, 4, 2, 1, 0, 2, 1, 0, 1, 2, 1, 0, 0, 2, 2, 3, 1, 1, 3, 6]
num_bins = 7
n, bins, patches = plt.hist(x, num_bins, facecolor='blue', alpha=0.5)
plt.show()
