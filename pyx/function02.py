import math
from pyx import *

def func(x):
    return math.log(x)

g = graph.graphxy(width=8)
g.plot(graph.data.function("y(x)=f(x)", min=-15, max=15, context={"f": func}))
g.writeEPSfile("function02")
g.writePDFfile("function02")
g.writeSVGfile("function02")
