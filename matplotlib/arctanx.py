import numpy as np
import matplotlib.pyplot as plt

x = np.linspace(-2, 2, 100)
y1 = x
y2 = x**2
y3 = x**3
y4 = x**4
y5 = x**5
y6 = np.arctan(x)
y7 = (1/2) * x**2
y8 = (1/3) * x**2
y9 = 1 - np.cos(x)

# plt.plot(x, y1, label='y = x')
plt.plot(x, y2, label='y = x**2')
# plt.plot(x, y3, label='y = x**3')
# plt.plot(x, y4, label='y = x**4')
# plt.plot(x, y5, label='y = x**5')
# plt.plot(x, y6, label='y = arctan(x)')
plt.plot(x, y7, label='y = (1/2) * x**2')
plt.plot(x, y8, label='y = (1/3) * x**2')
plt.plot(x, y9, label='y = 1 - cos(x)')

plt.ylim([-2, 2])
plt.legend()
plt.grid(True)
plt.show()
