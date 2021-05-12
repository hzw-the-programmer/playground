import matplotlib.ticker as tck
import matplotlib.pyplot as plt
import numpy as np

f,ax=plt.subplots(figsize=(20,10))
# x=np.linspace(-10*np.pi, 10*np.pi,1000)
x=np.linspace(-2*np.pi, 2*np.pi, 1000)
y=np.sin(x)

# ax.plot(x/np.pi,y)
ax.plot(x, y)

# ax.xaxis.set_major_formatter(tck.FormatStrFormatter('%g $\pi$'))
def func(x, pos):
    m = x / np.pi
    if m == 0:
        return '0'
    elif m == 1:
        return '$\pi$'
    elif m == -1:
        return '-$\pi$'
    return '{:.0f}$\pi$'.format(x/np.pi)
ax.xaxis.set_major_formatter(tck.FuncFormatter(func))
ax.xaxis.set_major_locator(tck.MultipleLocator(base=np.pi))
# start, end = ax.get_xlim()
# ax.xaxis.set_ticks(np.arange(start, end, np.pi))

plt.style.use("ggplot")

ax.grid(True)

plt.show()
