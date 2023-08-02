https://www.gnu.org/software/make/manual/html_node/Selective-Search.html
https://www.gnu.org/software/make/manual/html_node/General-Search.html
https://www.gnu.org/software/make/manual/html_node/Recipes_002fSearch.html

```
#
# VPATH Settings For The Specific Modules
#
vpath
vpath %.h        $(subst \,/,$(INCPATH))
vpath %.cpp      $(subst \,/,$(SRCPATH))
vpath %.cxx      $(subst \,/,$(SRCPATH))
vpath %.c        $(subst \,/,$(SRCPATH))
vpath %.dat      $(subst \,/,$(SRCPATH))
vpath %.s        $(subst \,/,$(SRCPATH))
vpath %.o        $(BUILD_DIR)/obj/$(COMPONENT)
vpath %.d        $(BUILD_DIR)/dep/$(COMPONENT)
vpath %.dep      $(BUILD_DIR)/dep

vpath %.a        $(TARGET_PATH)
```
