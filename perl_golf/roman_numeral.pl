%m=(M,1e3,D,5e2,C,1e2,L,50,X,10,V,5);map{$x=$p;$r+=($p=$m{$_}||1)>$x?-$x:$x}"@ARGV"=~/./g;print$r+$p;
