import numpy as np
import matplotlib.pyplot as plt

x = np.linspace(-2*np.pi, 2*np.pi, 100)
# plt.plot(x, x)
# plt.plot(x, np.sin(x))
# plt.plot(x, np.tan(x))
# plt.plot(x, np.exp(x))
# plt.plot(x, np.exp(x) - 1)
# plt.plot(x, np.log(x))
# plt.plot(x, np.log(1 + x))
# plt.plot(x, np.arcsin(x))
# plt.plot(x, np.arctan(x))

# x = np.linspace(-1.3, -0.001, 1000)
# l1 = plt.plot(x, x * np.sin(1 / x), label='y = x * sin(1 / x)')
# l2 = plt.plot(x, x**2 * np.sin(1 / x**2), label='y = x**2 * sin(1 / x**2)')
# # plt.plot(x, x**2.2 * np.sin(1 / x**2.2), label='x**2.2 * sin(1 / x**2.2)')
# l3 = plt.plot(x, x**3 * np.sin(1 / x**3), label='y = x**3 * sin(1 / x**3)')
# l4 = plt.plot(x, x**4 * np.sin(1 / x**4), label='y = x**4 * sin(1 / x**4)')
# x = np.linspace(0.001, 1.3, 1000)
# plt.plot(x, x * np.sin(1 / x), c=l1[0].get_color())
# plt.plot(x, x**2 * np.sin(1 / x**2), c=l2[0].get_color())
# # plt.plot(x, x**2.2 * np.sin(1 / x**2.2), label='x**2.2 * sin(1 / x**2.2)')
# plt.plot(x, x**3 * np.sin(1 / x**3), c=l3[0].get_color())
# plt.plot(x, x**4 * np.sin(1 / x**4), c=l4[0].get_color())

cmap = plt.get_cmap('jet_r')
N = 20
# x = np.linspace(-5, 5, 100)
# x = np.linspace(-10, 10, 100)
# x = np.linspace(-20, 20, 400)
x = np.linspace(0, 5, 100)
# x = np.linspace(0, 10, 100)
# x = np.linspace(0, 20, 100)
for i, n in enumerate(np.linspace(2.0, 0, N)):
    # y = n * x * np.sin(x)
    y = x**n * np.sin(x)
    color = cmap(i/N)
    lw = 2.5 if i==0 else 1
    # label = '%s * x * sin(x)' % n if i<3 else ''
    label = 'x**%s * sin(x)' % n if i<3 else ''
    plt.plot(x, y, c=color, lw=lw, zorder=-i, label=label)

plt.legend()
plt.grid(True)

plt.show()
