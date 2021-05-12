import numpy as np
import matplotlib.pyplot as plt

# x = np.linspace(-2, 2, 100)
# plt.plot(x, x**2, label='x**2')
# plt.plot(x, x**3, label='x**3')
# plt.plot(x, x**4, label='x**4')
# plt.plot(x, x**5, label='x**5')
# plt.plot(x, x**6, label='x**6')
# plt.plot(x, 2 * x**2, label='2*x**2')
# plt.plot(x, 3 * x**2, label='3*x**2')
# plt.plot(x, x**2 / 2, label='x**2/2')
# plt.plot(x, x**2 / 3, label='x**2/3')
# plt.plot(x, 2 * x**3, label='2*x**3')
# plt.plot(x, x**3 / 2, label='x**3/2')
# plt.ylim([-2, 2])

# x = np.linspace(-2, -0.1, 100)
# l1 = plt.plot(x, x**(-1), label='x**(-1)')
# l2 = plt.plot(x, x**(-2), label='x**(-2)')
# l3 = plt.plot(x, x**(-3), label='x**(-3)')
# l4 = plt.plot(x, x**(-4), label='x**(-4)')
# l5 = plt.plot(x, x**(-5), label='x**(-5)')
# l6 = plt.plot(x, x**(-6), label='x**(-6)')
# x = np.linspace(0.1, 2, 100)
# plt.plot(x, x**(-1), c=l1[0].get_color())
# plt.plot(x, x**(-2), c=l2[0].get_color())
# plt.plot(x, x**(-3), c=l3[0].get_color())
# plt.plot(x, x**(-4), c=l4[0].get_color())
# plt.plot(x, x**(-5), c=l5[0].get_color())
# plt.plot(x, x**(-6), c=l6[0].get_color())
# plt.ylim([-10, 10])

# x = np.linspace(0, 4, 400)
# plt.plot(x, x**(1/2))
# plt.plot(x, x**(1/3))
# plt.plot(x, x**(1/4))
# plt.plot(x, x)

cmap = plt.get_cmap('jet_r')
N = 20
x = np.linspace(-2, 2, 100)
for i, n in enumerate(np.linspace(2.0, 0, N)):
    # y = n * x**2
    y = n * x**3
    # y = n * x**4
    color = cmap(i/N)
    lw = 2.5 if i==10 else 1
    # label = '%s * x**2' % n if i==10 else ''
    label = '%s * x**3' % n if i==10 else ''
    # label = '%s * x**4' % n if i==10 else ''
    plt.plot(x, y, c=color, lw=lw, zorder=-i, label=label)

# cmap = plt.get_cmap('jet_r')
# N = 20
# x = np.linspace(-2, -0.01, 100)
# for i, n in enumerate(np.linspace(2.0, 0, N)):
#     # y = n * x**(-1)
#     # y = n * x**(-2)
#     # y = n * x**(-3)
#     y = n * x**(-4)
#     color = cmap(i/N)
#     lw = 2.5 if i==10 else 1
#     # label = '%s * x**(-1)' % n if i==10 else ''
#     # label = '%s * x**(-2)' % n if i==10 else ''
#     # label = '%s * x**(-3)' % n if i==10 else ''
#     label = '%s * x**(-4)' % n if i==10 else ''
#     plt.plot(x, y, c=color, lw=lw, zorder=-i, label=label)
# x = np.linspace(0.01, 2, 100)
# for i, n in enumerate(np.linspace(2.0, 0, N)):
#     # y = n * x**(-1)
#     # y = n * x**(-2)
#     # y = n * x**(-3)
#     y = n * x**(-4)
#     color = cmap(i/N)
#     lw = 2.5 if i==10 else 1
#     plt.plot(x, y, c=color, lw=lw, zorder=-i)
# plt.ylim([-2, 2])

plt.grid(True)
plt.legend()

plt.show()
