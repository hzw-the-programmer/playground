grep Register -r --exclude-dir={vendor,website} --exclude={*test*,*pb*} .
grep -rn "connection refused" .

grep -r --exclude-dir=Debug "PhbSrv" MoDIS_VC9/

grep -r --exclude-dir={build,MoDIS_VC9} "HZW" .

grep "xxx" -r --exclude-dir={Bin} application/

grep "B(efore) 1 line and A(fter) 1 line" -B 1 -A 1 --color -r .

grep text -ri --color docs
