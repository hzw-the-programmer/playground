import fileinput

stat = {}
for line in fileinput.input():
    (did, pid) = line.split()
    if did in stat and stat[did] != pid:
        raise()
    else:
        stat[did] = pid

for k in sorted(stat.keys(), key=int):
    print(k, stat[k])
