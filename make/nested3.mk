x = var1
var2 = hello
y = $(subst 1,2,$(x))
z = y
$(info $($($(z))))