@x="@ARGV"=~/./g;%m=('M',1e3,'D',5e2,'C',1e2,'L',50,'X',10,'V',5);map $r+=($v=$m{shift @x}||1)<$m{$x[0]}?-$v:$v,@x;print $r;
