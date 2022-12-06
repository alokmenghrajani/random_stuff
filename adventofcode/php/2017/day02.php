<?php
ini_set('memory_limit','500M');

function idx($arr, $el, $default) {
  if (!isset($arr[$el])) {
    return $default;
  }
  return $arr[$el];
}

$lines = trim(file_get_contents('day02.txt'));
$lines = explode("\n", $lines);

$sum = 0;
foreach ($lines as $k => $line) {
  $numbers = preg_split('/\s+/', $line);
  $k = 0;
  foreach ($numbers as $i1 => $n1) {
    foreach ($numbers as $i2 => $n2) {
      if ($i2 == $i1) {
        continue;
      }
      echo $n1, " ", $n2, "\n";
        if ($n1 % $n2 == 0) {
          $k = $n1 / $n2;
        }
    }
  }
  echo $k, "\n";
  $sum += $k;
}
echo $sum;
