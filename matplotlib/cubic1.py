import numpy as np
import matplotlib.pyplot as plt

x = np.linspace(-2, 2, 100)
plt.plot(x, x, label='linear')
plt.plot(x, x**2, label='quadratic')
plt.plot(x, x**3, label='cubit')

plt.legend()

plt.xlabel('x label')
plt.ylabel('y label')
plt.title('Simple Plot')

plt.grid(True)

plt.show()
