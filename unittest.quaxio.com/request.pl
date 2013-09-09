#!/usr/bin/perl

print("Content-type: text/plain\n");
print("\n");
foreach (sort keys %ENV)
{
  print "$_: $ENV{$_}\n";
}
