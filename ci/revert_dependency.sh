#!/bin/sh

path=$0

if [[ -z $1 ]]; then
    echo "missing argument 1 (dependency name)"
    exit -1
fi

dep=$1

if [ -e Cargo.toml ]; then
    echo "Updating Cargo.toml for development"
else
    echo "No Cargo.toml in current directory."
    exit -1
fi

sed -r "s/^$dep(.*)/### $dep\\1/g" Cargo.toml >temp.toml
sed -r "s/^## $dep(.*)/$dep\\1/g" temp.toml >Cargo.toml
rm temp.toml

echo "Success"
cat Cargo.toml
