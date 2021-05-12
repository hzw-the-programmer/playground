from math import pi, sin, cos
from pyx import *

g = graph.graphxy(width=8)
g.plot(graph.data.paramfunctionxy(lambda k: (sin(2*k), cos(3*k)), 0, 2*pi))
# g.writeEPSfile()
g.writePDFfile()
# g.writeSVGfile()
