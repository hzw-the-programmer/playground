grep Register -r --exclude-dir={vendor,website} --exclude={*test*,*pb*} .
grep -rn "connection refused" .

grep -r --exclude-dir=Debug "PhbSrv" MoDIS_VC9/

grep -r --exclude-dir={build,MoDIS_VC9} "HZW" .

grep "xxx" -r --exclude-dir={Bin} application/
