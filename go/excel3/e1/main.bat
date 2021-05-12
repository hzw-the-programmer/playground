@echo off
SET major=1
SET minor=0

main.exe -inputfile=translations.xlsx -sheet=s1 -embed -gzip -major=%major% -minor=%minor%
main.exe -inputfile=translations.xlsx -sheet=s1,s2,s3 -gzip -enum -major=%major% -minor=%minor%
