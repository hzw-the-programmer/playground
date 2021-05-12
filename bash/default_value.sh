#!/usr/bin/env bash

a=1

echo 'a=' $a
echo ':-' ${a:-2}
echo ' -' ${a-2}
echo ':=' ${a:=2}
echo ' =' ${a=2}
echo ':?' ${a:?2}
echo ' ?' ${a?2}
echo ':+' ${a:+2}
echo ' +' ${a+2}
echo 'a=' $a
echo

echo 'b=' $b
echo ':-' ${b:-2}
echo ' -' ${b-3}
echo ':=' ${b:=4}
unset b
echo ' =' ${b=5}
unset b
#echo ':?' ${b:?6}
#echo ' ?' ${b?7}
echo ':+' ${b:+8}
echo ' +' ${b+8}
echo 'b=' $b
echo