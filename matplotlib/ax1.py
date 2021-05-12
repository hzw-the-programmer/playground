import numpy as np
import matplotlib.pyplot as plt

# x = np.linspace(-2, 2, 100)
# plt.plot(x, x)
# plt.plot(x, np.exp(x))
# plt.plot(x, np.exp(x) - 1)

# plt.plot(x, x*np.log(3))
# plt.plot(x, 3**x)
# plt.plot(x, 3**x - 1)

# plt.plot(x, x*np.log(4))
# plt.plot(x, 4**x)
# plt.plot(x, 4**x - 1)

# plt.plot(x, x*np.log(5))
# plt.plot(x, 5**x)
# plt.plot(x, 5**x - 1)

# plt.plot(x, x*np.log(2))
# plt.plot(x, 2**x)
# plt.plot(x, 2**x - 1)

# plt.plot(x, x*np.log(1/2))
# plt.plot(x, (1/2)**x)
# plt.plot(x, (1/2)**x - 1)

cmap = plt.get_cmap('jet_r')
N = 20
x = np.linspace(-2, 2, 100)
for i, n in enumerate(np.linspace(2.0, 0, N)):
    y = n * np.exp(x)
    color = cmap(i/N)
    lw = 2.5 if i==10 else 1
    label = '%s * exp(x)' % n if i==10 else ''
    plt.plot(x, y, c=color, lw=lw, zorder=-i, label=label)

plt.ylim([-2, 2])
plt.grid(True)
plt.show()
