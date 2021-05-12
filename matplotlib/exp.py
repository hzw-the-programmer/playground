import numpy as np
import matplotlib.pyplot as plt

x = np.linspace(-1, 2, 100)
plt.plot(x, np.exp(x))
# plt.plot(x, -np.exp(x))
# plt.plot(x, np.exp(-x))

# plt.plot(x, np.exp(x) + 1)
# plt.plot(x, np.exp(x) - 1)
# plt.plot(x, np.exp(x) + 2)
# plt.plot(x, np.exp(x) - 2)

plt.plot(x, np.exp(x + 1))
plt.plot(x, np.exp(x - 1))
plt.plot(x, np.exp(x + 2))
plt.plot(x, np.exp(x - 2))

# plt.plot(x, 2*np.exp(x))
# plt.plot(x, np.exp(x)/2)
# plt.plot(x, 3*np.exp(x))
# plt.plot(x, np.exp(x)/3)


# plt.plot(x, np.exp(2*x))
# plt.plot(x, np.exp(x/2))
# plt.plot(x, np.exp(3*x))
# plt.plot(x, np.exp(x/3))

plt.ylim([0, 8])

plt.grid(True)

plt.show()
