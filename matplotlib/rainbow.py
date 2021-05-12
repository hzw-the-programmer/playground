import numpy as np
import matplotlib.pyplot as plt

x = np.linspace(0, 5, 100)
cmap = plt.get_cmap('jet_r')
N = 20
for i, n in enumerate(np.linspace(2.0, 0, N)):
    y = np.sin(x)*x**n
    color = cmap(float(i)/N)
    lw = 2.5 if i==0 else 1
    # zorder controls the order in which the lines are drawn.
    # The line with the highest zorder lies on top.
    plt.plot(x, y, c=color, lw=lw, zorder=-i)
plt.show()
