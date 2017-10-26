#!/usr/bin/env bash
function panic() {
	echo $@ >&2
	exit 1
}

rootdir=`dirname $0`

protoc --version &>/dev/null || panic protoc is required.
output=(`protoc --version`)
version=${output[1]}
IFS=. read major minor patch <<<$version
if [[ $major -lt 3 ]]; then
	panic expect protoc 3.0.0+ but got $version.
fi

while read include package name; do
	prefix="$rootdir/proto/$package"
	include="$rootdir/proto/$include"
	echo building $name.desc
	find "$prefix" -name "*.proto" | xargs protoc -I "$include" -o "$rootdir/$name.desc"
done <<EOF
. grpc/testing testing
. grpc/health/v1/ health
example example example
EOF
