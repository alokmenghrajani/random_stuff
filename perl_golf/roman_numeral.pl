@x="@ARGV"=~/./g;%m=('M',1e3,'D',500,'C',100,'L',50,'X',10,'V',5,'I',1);$r+=($v=$m{shift @x})<$m{$x[0]}?-$v:$v for 0..14;print $r;
