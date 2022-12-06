<?php

function process($a) {
  $r = $a;
  $r = $r . '0';
  for ($i=strlen($a)-1; $i>=0; $i--) {
    $r = $r . ($a[$i] == '1' ? '0' : '1');
  }
  return $r;
}

function checksum($s) {
  if (strlen($s) % 2 == 1) {
    return $s;
  }
  $r = '';
  $l = strlen($s);
  for ($i=0; $i<$l; $i+=2) {
    $t = substr($s, $i, 2);
    if ($t[0] == $t[1]) {
        $r = $r . '1';
      } else {
        $r = $r . '0';
      }
  }
  echo 'here: ', $r, "\n";
  return checksum($r);
}

$start = '01111001100111011';
while (strlen($start) < 35651584) {
  $start = process($start);
}
$start = substr($start, 0, 35651584);

//$start = '110010110100';

$c = checksum($start);
echo 'checksum: ', $c, "\n";



// 10000011110010000111110
// 10000011110010000111110

// $lines = trim(file_get_contents('day16.txt'));
// $lines = split("\n", $lines);
// foreach ($lines as $k => $v) {
//    echo "x: ", $k, " ", $v, "\n";
// }
