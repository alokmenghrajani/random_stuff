#!/usr/bin/perl

print("HTTP/1.1 302 Moved Temporarily\n");
print("Content-length: 0\n");
print("Location: http://unittest.quaxio.com/request.pl?foo=value with spaces\n");
print("Content-Type: text/html; charset=UTF-8\n");
print("\n");
