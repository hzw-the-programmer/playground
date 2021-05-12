#!/usr/bin/env bash
echo $0 $1 $2
echo $#

echo $@
echo "$@"

echo $*
echo "$*"

shift

echo $0 $1 $2
echo $#

echo $IFS
IFS=,

echo $@
echo "$@"

echo $*
echo "$*"

echo ${BASH_SOURCE-123}
echo ${bash_source-123}

echo ${BASH_SOURCE:-123}
echo ${bash_source:-123}
