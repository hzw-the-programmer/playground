https://www.gnu.org/software/make/manual/html_node/Wildcard-Function.html
https://www.gnu.org/software/make/manual/html_node/Text-Functions.html#Text-Functions
https://www.gnu.org/software/make/manual/html_node/File-Name-Functions.html
$(notdir src/foo.c hacks)

```
mkdir MS_Code/MS_MMI/source/demo/a
mkdir MS_Code/MS_MMI/source/demo/b
mkdir MS_Code/MS_MMI/source/demo/c

vi MS_Code/MS_MMI/source/demo/a/a.c
vi MS_Code/MS_MMI/source/demo/a/a.h
vi MS_Code/MS_MMI/source/demo/b/b.c
vi MS_Code/MS_MMI/source/demo/b/b.h
vi MS_Code/MS_MMI/source/demo/c/c.c
vi MS_Code/MS_MMI/source/demo/c/c.h

mkdir MS_Code/make/demo
vi MS_Code/make/demo/demo.mk
```

```
DEMOSRCPATH += MS_MMI/source/demo/a
DEMOSRCPATH += MS_MMI/source/demo/b
DEMOSRCPATH += MS_MMI/source/demo/c

MSRCPATH += $(DEMOSRCPATH)
SOURCES += $(foreach dir, ${DEMOSRCPATH}, $(patsubst ${dir}/%.c,%.c,$(wildcard ${dir}/*.c)))
```
