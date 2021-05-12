import numpy as np
import matplotlib.pyplot as plt

cmap = plt.get_cmap('jet_r')
N = 20
x = np.linspace(0.01, 2, 100)
for i, n in enumerate(np.linspace(2.0, 0, N)):
    y = n * np.log(x)
    color = cmap(i/N)
    lw = 2.5 if i==10 else 1
    label = '%s * ln(x)' % n if i==10 else ''
    plt.plot(x, y, c=color, lw=lw, zorder=-i, label=label)

plt.ylim([-2, 2])
plt.grid(True)
plt.show()
