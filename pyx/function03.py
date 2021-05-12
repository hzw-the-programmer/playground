import math
from pyx import *

def func(x):
    return math.floor(x)

g = graph.graphxy(width=8)
# g.plot(graph.data.function("y(x)=f(x)", min=-3, max=3, context={"f": func}, points=10), [graph.style.symbol()])
# g.plot(graph.data.function("y(x)=f(x)", min=-3, max=3, context={"f": func}, points=10))
g.plot(graph.data.function("y(x)=f(x)", min=-3, max=3, context={"f": func}, points=1000))
# g.writeEPSfile()
g.writePDFfile()
# g.writeSVGfile()
