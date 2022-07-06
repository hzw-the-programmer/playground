comma := ,
empty := 
space := $(empty) $(empty)
foo := a b c
# bar := $(subst $(space),$(comma), $(foo))
bar := $(subst $(space),$(comma),$(foo))
$(info $(bar))