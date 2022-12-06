<?php

function unhex($c) {
  if (is_numeric($c)) {
    return $c+0;
  }
  return ord($c) - ord('a') + 10;
}

function md($current, $arr) {
//  echo "here: ", $current, " ", join("-", $arr), "\n";
  $s = '';
  for ($i=0; $i<count($arr); $i++) {
    $s .= $arr[$i] . '-';
  }
  $new = md5($s);
  for ($i=0; $i<strlen($new); $i++) {
    $t = dechex(unhex($current[$i]) ^ unhex($new[$i]));
    $current[$i] = $t;
  }
  return $current;
}

//$a = [1, 2, 3, 4, 5, 6, 7, 9, 8];
$a = [1, 3, 4, 5, 6, 7, 2, 8, 9];

$hash = '00000000000000000000000000000000';
$hash = md($hash, $a);
$done = false;
$n = 0;
while (!$done) {
  if ($n % 1000 == 0) {
    echo join('-', $a), " ", $n, "\n";
  }
  $done = true;
  for ($i=0; $i<count($a)-1; $i++) {
    if ($a[$i] < $a[$i+1]) {
      $done = false;
    }
  }
  if (!$done) {
    $i = mt_rand(0, count($a)-1);
    $j = mt_rand(0, count($a)-2);
    if ($j >= $i) {
      $j++;
    }
    $hash = md($hash, $a);
    $a[$i] = $a[$i] ^ $a[$j];
    $a[$j] = $a[$j] ^ $a[$i];
    $a[$i] = $a[$i] ^ $a[$j];
    $hash = md($hash, $a);
    $n++;
  }
}
echo $hash, "\n";
echo $n, "\n";
print_r($a);
