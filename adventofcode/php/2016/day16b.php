<?php

function process($a) {
  if (strlen($a) % 2 == 1) {

  } else {
  $r = checksum2($a);
  $r = $a;
  $r = $r . '0';
  for ($i=strlen($a)-1; $i>=0; $i--) {
    $r = $r . ($a[$i] == '1' ? '0' : '1');
  }
  return $r;
}

function checksum2($s) {
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
$true_len = strlen($start);
while ($true_len < 35651584) {
  $start = process($start);
  $true_len = $true_len*2 + 1;
  echo 'len: ', $true_len, " ", strlen($start), "\n";
}
echo "done part 1\n";
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
