from math import pi, sin, cos
from pyx import *

def lissajousdata(k):
    return sin(2*k), cos(3*k)

g = graph.graphxy(width=8)
g.plot(graph.data.paramfunction("k", 0, 2*pi, "x, y = f(k)", context={"f": lissajousdata}))
g.writeEPSfile("lissajous01")
g.writePDFfile("lissajous01")
g.writeSVGfile("lissajous01")
