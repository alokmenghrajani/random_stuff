#!/usr/bin/perl
my $file = 'spacer.gif';
open(FH,'<', $file) || die "Could not open $file: $!";
my $length = -s $file;
print("Content-type: image/gif\n");
print("Content-length: $length\n");
print("Set-Cookie: large=" . "w" x 1025 . "; path=/\n\n");
binmode STDOUT;
while(<FH>){print $_;}
close(FH);
