from pyx import *

g = graph.graphxy(width=8)
g.plot(graph.data.file("minimal01.dat"), [graph.style.line()])
g.writeEPSfile("minimal01")
g.writePDFfile("minimal01")
g.writeSVGfile("minimal01")
