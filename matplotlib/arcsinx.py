import numpy as np
import matplotlib.pyplot as plt

fig, ax = plt.subplots()

ax.axhline(y=(-1/2) * np.pi, c='r')

x1 = np.linspace(-1, 1, 400)
y1 = np.arcsin(x1)
ax.plot(x1, y1, label='y = arcsin(x)')
x2 = np.linspace(-4, 4)
y2 = np.arctan(x2)
ax.plot(x2, y2, label='y = arctan(x)')
ax.plot(x2, x2, label='y = x')

ax.axhline(y=(1/2) * np.pi, c='r')

ax.legend()
ax.grid(True)

plt.show()
