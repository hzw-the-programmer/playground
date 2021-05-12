import numpy as np
import matplotlib.pyplot as plt

fig, ax = plt.subplots()

ax.axvline(x=(-3/2) * np.pi, c='r')

x = np.linspace((-3/2) * np.pi + 0.01, -(1/2) * np.pi - 0.01, 100)
y1 = np.tan(x)
l1 = ax.plot(x, y1, label='y = tan(x)')

ax.axvline(x=(-1/2) * np.pi, c='r')

x = np.linspace((-1/2) * np.pi + 0.01, (1/2) * np.pi - 0.01, 100)
ax.plot(x, y1, c=l1[0].get_color())

ax.axvline(x=(1/2) * np.pi, c='r')

x = np.linspace((1/2) * np.pi + 0.01, (3/2) * np.pi - 0.01, 100)
ax.plot(x, y1, c=l1[0].get_color())

ax.axvline(x=(3/2) * np.pi, c='r')

x = np.linspace(-2 * np.pi, 2 * np.pi, 100)
y2 = np.sin(x)
ax.plot(x, y2, label='y = sin(x)')
ax.plot(x, x, label='y = x')

ax.set_ylim(-4, 4)
ax.legend()
ax.grid(True)
plt.show()
