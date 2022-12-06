<?php
ini_set('memory_limit','500M');

function idx($arr, $el, $default) {
  if (!isset($arr[$el])) {
    return $default;
  }
  return $arr[$el];
}

$state = "14,0,15,12,11,11,3,5,1,6,8,4,9,1,8,4";
           //"0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0"
//$state =   "0,2,7,0,0,0,0,0,0,0,0,0,0,0,0,0";
$seen = array();
$steps = 0;
while (1) {
  echo $state, "\n";
  $new_state = process($state);
  $steps++;
  if (isset($seen[$new_state])) {
    echo $steps, "\n";
    echo $seen[$new_state], "\n";
    die;
  }
  $state = $new_state;
  $seen[$state] = $steps;
}

function process($str) {
  $a = explode(",", $str);
  // find max
  $idx = 0;
  $max = 0;
  for ($i=0; $i<count($a); $i++) {
    if ($a[$i] > $max) {
      $idx = $i;
      $max = $a[$i];
    }
  }
  // redistribute
  $a[$idx] = 0;
  for ($k=1; $k<=$max; $k++) {
    $a[($idx + $k) % count($a)]++;
  }
  return join($a, ",");
}


//echo substr(md5("hello world"), 0, 5);

$lines = trim(file_get_contents('day06.txt'));
$lines = explode("\n", $lines);

$a = [];
foreach ($lines as $k => $line) {
  echo "x: ", $k, " ", $line, "\n";
  $matches = [];
  if (preg_match('/^(\S+),\s*(\d+)$/', $line, $matches)) {
    array_shift($matches);
    $a[] = $matches;
  }
}

print_r($a);
