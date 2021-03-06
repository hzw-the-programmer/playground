from pyx import *

g = graph.graphxy(width=8)
# either provide lists of the individual coordinates
# g.plot(graph.data.values(x=list(range(10)), y=list(range(10))))
# or provide one list containing the whole points
g.plot(graph.data.points(list(zip(range(10), range(10))), x=1, y=2), [graph.style.line()])
g.writeEPSfile("points01")
g.writePDFfile("points01")
g.writeSVGfile("points01")
