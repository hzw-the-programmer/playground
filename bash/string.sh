#!/usr/bin/env bash
str=abcABC123ABCabc

echo ${str:0}
echo ${str:1}
echo ${str:7}
echo ${str:7:3}

echo ${str:-4}
echo ${STR:-4} #default value
echo ${str:(-4)}
echo ${str: -4}