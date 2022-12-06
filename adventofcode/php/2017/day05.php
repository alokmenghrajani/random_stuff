<?php
ini_set('memory_limit','500M');

function idx($arr, $el, $default) {
  if (!isset($arr[$el])) {
    return $default;
  }
  return $arr[$el];
}

//echo substr(md5("hello world"), 0, 5);

$lines = trim(file_get_contents('day05.txt'));
$lines = explode("\n", $lines);
//$lines = [0, 3, 0, 1, -3];

$ptr = 0;
$steps = 0;
while (1) {
  if (($ptr < 0) || ($ptr >= count($lines))) {
    echo 'steps: ', $steps, "\n";
    break;
  }
  $t = (int)$lines[$ptr];
  //echo 'here: ', $t, "\n";
  if ($t >= 3) {
    $lines[$ptr] = $lines[$ptr] - 1;
  } else {
    $lines[$ptr] = $lines[$ptr] + 1;
  }
  $ptr += $t;
  $steps++;
}

die;
$a = [];
$sum = 0;
foreach ($lines as $k => $line) {
  echo "x: ", $k, " ", $line, "\n";
  $matches = [];
  if (preg_match('/^(\S+),\s*(\d+)$/', $line, $matches)) {
    array_shift($matches);
    $a[] = $matches;
  }
}

echo "sum: ", $sum, "\n";
print_r($a);
