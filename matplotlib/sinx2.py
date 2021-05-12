import numpy as np
import matplotlib.pyplot as plt

x = np.linspace(-2*np.pi, 2*np.pi, 100)
plt.plot(x, np.sin(x))
# plt.plot(x, -np.sin(x))
# plt.plot(x, np.sin(-x))
# plt.plot(x, np.sin(x) + 1)
# plt.plot(x, np.sin(x) - 1)
# plt.plot(x, np.sin(x + 1))
# plt.plot(x, np.sin(x - 1))
# plt.plot(x, 2*np.sin(x))
# plt.plot(x, np.sin(x)/2)
plt.plot(x, np.sin(2*x))
plt.plot(x, np.sin(x/2))

plt.grid(True)

plt.show()
