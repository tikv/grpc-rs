#!/usr/bin/env bash
function panic() {
	echo $@ >&2
	exit 1
}

function exists () {
    which "$1" 1>/dev/null 2>&1
}

function push () {
    pushd $1 >/dev/null 2>&1
}

function pop () {
    popd $1 >/dev/null 2>&1
}

rootdir=`dirname $0`

protoc --version &>/dev/null || panic protoc is required.
output=(`protoc --version`)
version=${output[1]}
IFS=. read major minor patch <<<$version
if [[ $major -lt 3 ]]; then
	panic expect protoc 3.0.0+ but got $version.
fi

exists protoc-gen-rust || panic rust-protobuf protoc is required
exists grpc_rust_plugin || panic grpc_rust_plugin is required

lib_rs=$rootdir/src/lib.rs

rm -f $lib_rs
echo '// Copyright 2017 PingCAP, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate futures;
extern crate grpcio;
extern crate protobuf;

pub mod example;
pub mod grpc;

pub mod util;' > $lib_rs

while read include package name; do
	prefix="$rootdir/proto/$package"
	include="$rootdir/proto/$include"
	output="$rootdir/src/$package"
	echo building package $package
	mkdir -p $output
	find "$prefix" -name "*.proto" | xargs protoc -I "$include" --rust_out $output
	find "$prefix" -name "*.proto" | xargs protoc -I "$include" --grpc_out $output --plugin=protoc-gen-grpc=`which grpc_rust_plugin`
	push $output
	rm -f mod.rs
	for file in `ls *.rs`
    do
		base_name=$(basename $file ".rs")
		echo "pub mod $base_name;" >> mod.rs
	done
	pop
done <<EOF
. grpc/core core
. grpc/testing testing
. grpc/health/v1/ health
example example example
EOF

# Write mod.rs for mod grpc
grpc_dirs=$(find $rootdir/src/grpc -type d)
for gdir in $grpc_dirs
do
	mods=$(find $gdir -mindepth 1 -maxdepth 1 -type d | egrep '.*')
	[ $? -ne 0 ] && continue
	push $gdir
	rm -f mod.rs
	for mod in $mods
    do
		base_name=$(basename $mod)
		echo "pub mod $base_name;" >> mod.rs
	done
	pop
done
