#!/usr/bin/perl

# Imports
use strict;
use warnings FATAL => 'all';
use IPC::Open3;
use File::Which;

# Stop Perl from buffering (unimportant implementation detail)
$|++;

# Find: clang-tidy, clang-tidy-[4-6].0 in Path environment variable
my $clangTidy = which 'clang-tidy';
for my $ver (4 .. 6) {
    $clangTidy = which "clang-tidy-$ver.0" if !defined $clangTidy;
}
die "Cannot find clang-tidy!" if !defined $clangTidy;

# Variable definitions (path related)
# Put together so people can go and modify without reading the whole script
my $grpcSysPath = './grpc-sys';
my $grpcPath = "$grpcSysPath/grpc";
my $clangCompletePath = "$grpcPath/.clang_complete";
my $extraArgFlag = ' -extra-arg-before=';

# Open `.clang_complete` in grpc
open my $clangComplete, '<', $clangCompletePath or die "Failed to open $clangCompletePath: $!\n";

# Setup extra arguments
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

# Concatenate the command line arguments to clang-tidy
my $completeArgs = "-checks=clang-analyzer-*$extraArgFlag@{[ join $extraArgFlag, @extraArgs ]} $grpcSysPath/grpc_wrap.cc";

# Debug information
print "Running $clangTidy with: $completeArgs\n";

# Start clang-tidy process
no warnings 'once';
my $pid = open3 \*WRITER, \*READER, \*ERROR, "$clangTidy $completeArgs";

# Print clang-tidy's stdout to current process' stdout
print "\nClang-Tidy stdout:\n";
while ($_ = <READER>) {
    print;
}

# Print clang-tidy's stderr to current process' stderr
print "\nClang-Tidy stderr:\n";
while ($_ = <ERROR>) {
    print;
}

# Wait the clang-tidy process to end
waitpid $pid, 0 or die "$!\n";

# Exit with the return value of clang-tidy's
exit $?;
