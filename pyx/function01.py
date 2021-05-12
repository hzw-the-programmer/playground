from pyx import *

g = graph.graphxy(width=8, x=graph.axis.linear(min=-15, max=15))
g.plot(graph.data.function("y(x)=sin(x)/x"))

# g = graph.graphxy(width=8)
# g.plot(graph.data.function("x(y)=sin(y)/y", min=-15, max=15))

# g = graph.graphxy(width=8, y=graph.axis.linear(min=-15, max=15))
# g.plot(graph.data.function("x(y)=sin(y)/y"))

g.writeEPSfile("function01")
g.writePDFfile("function01")
g.writeSVGfile("function01")
