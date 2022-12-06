<?php

$door = "ojvtpuvg";

$i=0;
$found = 0;
$r = '--------';
while ($found < 20) {
  $t = check($door, $i);
  if ($t !== false) {
    $r[$t[0]] = $t[1];
    echo "r=", $r, "\n";
    $found++;
  }
  $i++;
}

function check($str, $n) {
  $t = md5($str . $n);
  if (strcmp(substr($t, 0, 5), "00000") == 0) {
    if ($t[5] == '0' ||
    $t[5] == '1' ||
    $t[5] == '2' ||
    $t[5] == '3' ||
    $t[5] == '4' ||
    $t[5] == '5' ||
    $t[5] == '6' ||
    $t[5] == '7') {
      return [$t[5], $t[6]];
    }
  }
  return false;
}
