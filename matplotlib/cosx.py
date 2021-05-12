import numpy as np
import matplotlib.pyplot as plt

x = np.linspace(-2*np.pi, 2*np.pi, 100)

# plt.plot(x, 1 - np.cos(x))
# plt.plot(x, x**2)
# plt.plot(x, 2*x**2)
# plt.plot(x, x**2/2)
# plt.plot(x, x**2/3)

plt.plot(x, np.cos(x))

# plt.plot(x, -np.cos(x))
# plt.plot(x, np.cos(-x))

# plt.plot(x, np.cos(x) + 1)
# plt.plot(x, np.cos(x) - 1)

# plt.plot(x, np.cos(x + 1))
# plt.plot(x, np.cos(x - 1))

# plt.plot(x, 2 * np.cos(x))
# plt.plot(x, np.cos(x) / 2)
plt.plot(x, np.cos(2 * x))
plt.plot(x, np.cos(x / 2))

plt.ylim([-2, 2])

plt.grid(True)
plt.show()
