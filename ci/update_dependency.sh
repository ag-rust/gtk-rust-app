#!/bin/sh

path=$0

if [[ -z $1 ]];
then
echo "missing argument 1 (dependency name)"
exit -1
fi

if [[ -z $2 ]];
then
echo "missing argument 2 (latest version)"
exit -1
fi

if [[ -z $3 ]];
then
echo "missing argument 2 (next version)"
exit -1
fi

dep=$1
v_from=$2
v_to=$3

if [ -e Cargo.toml ]
then
    echo "Updateing Cargo.toml"
else
    echo "No Cargo.toml in current directory."
    exit -1
fi

sed -r "s/$dep(.*)/## $dep\\1/g" Cargo.toml > temp.toml
sed -r "s/## ## $dep(.*?)($v_from)(.*?)/$dep\\1$v_to\\3/g" temp.toml > Cargo.toml
rm temp.toml

echo "Success"
cat Cargo.toml
