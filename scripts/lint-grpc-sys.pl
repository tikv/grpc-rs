#!/usr/bin/perl

use strict;
use warnings FATAL => 'all';
use IPC::Open3;

$|++;

my $grpcSysPath = './grpc-sys';
my $grpcPath = "$grpcSysPath/grpc";
my $clangCompletePath = "$grpcPath/.clang_complete";
my $extraArgFlag = ' -extra-arg-before=';
open my $clangComplete, '<', $clangCompletePath or die "Failed to open $clangCompletePath: $!\n";

my @extraArgs = (
    "-x",
    "c++",
    "-std=c++11",
);

while ($_ = <$clangComplete>) {
    chomp;
    push @extraArgs, s/^-I(.*)/-I$grpcPath\/$1/r if /^-I/;
}

close $clangComplete or die "Failed to close $clangCompletePath: $!\n";

my $completeArgs = "-checks=clang-analyzer-*$extraArgFlag@{[ join $extraArgFlag, @extraArgs ]} $grpcSysPath/grpc_wrap.cc";

print "Running clang-tidy with: $completeArgs\n";

no warnings 'once';
my $pid = open3 \*WRITER, \*READER, \*ERROR, "clang-tidy-6.0 $completeArgs";

print "\nClang-Tidy stdout:\n";
while ($_ = <READER>) {
    print;
}

print "\nClang-Tidy stderr:\n";
while ($_ = <ERROR>) {
    print;
}

waitpid $pid, 0 or die "$!\n";

exit $?;
