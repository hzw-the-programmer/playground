import numpy as np
import matplotlib.pyplot as plt

x = np.linspace(-2, 2, 100)

plt.plot(x, np.exp(x))
plt.plot(x, 2**x)
plt.plot(x, 3**x)
plt.plot(x, (1/2)**x)
plt.plot(x, (1/3)**x)
plt.plot(x, np.exp(-x))

plt.ylim([0, 2])
plt.grid(True)

plt.show()
