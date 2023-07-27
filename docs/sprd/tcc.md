c:\tmp>tcc --cpu ARM926EJ-S -O3 --apcs /interwork/ --diag_suppress 1,9,61,66,68,111,117,144,152,167,170,174,177,186,188,191,223,226,236,494,
513,550,940,1134,1287,1294,1295,1296,1381,1441,1608,1652,1764,1786,3017,3052 --fpu softvfp --enum_is_int -D_RTOS -D_DEBUG --loose_implicit_c
ast --debug --li -g -c util.c