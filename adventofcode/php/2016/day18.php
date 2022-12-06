<?php

$arr = [];

function idx($arr, $key, $value) {
  if (!isset($arr[$key])) {
    return $value;
  }
  return $arr[$key];
}

function is_trap($i, $j, $prev) {
  global $arr;
  $l = idx($prev, $j-1, 0);
  $c = idx($prev, $j, 0);
  $r = idx($prev, $j+1, 0);

  if ($l && $c && !$r) {
    return 1;
  }
  if (!$l && $c && $r) {
    return 1;
  }
  if ($l && !$c && !$r) {
    return 1;
  }
  if (!$l && !$c && $r) {
    return 1;
  }
  return 0;
}

function c_to_n($c) {
  if ($c == '.') {
    return 0;
  }
  if ($c == '^') {
    return 1;
  }
}
function print_row($row) {
  for ($i=0; $i<count($row); $i++) {
    echo $row[$i] ? '^' : '.';
  }
  echo "\n";
}
//$arr = [[0, 0, 1, 1, 0]];
//$arr = [[0, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1]];
//$arr = [[0, 1, 1, 0, 1, 0, 1, 1, 1, 1]];
//$t = ".^^.^.^^^^";
$t = ".^^^^^.^^.^^^.^...^..^^.^.^..^^^^^^^^^^..^...^^.^..^^^^..^^^^...^.^.^^^^^^^^....^..^^^^^^.^^^.^^^.^^";
$row = [];
$n = 0;
for ($i=0; $i<strlen($t); $i++) {
  $row[$i] = c_to_n($t[$i]);
  $n += !$row[$i];
}

$size = count($arr[0]);
for ($i = 1; $i<400000; $i++) {
  $next = [];
  for ($j=0; $j<$size; $j++) {
    $next = is_trap($i, $j, $row);
    $n += !$next[$j];
  }
  $row = $next;
  //print_row($arr[$i]);
}
echo $n;

// $lines = trim(file_get_contents('day18.txt'));
// $lines = split("\n", $lines);
//
// foreach ($lines as $k => $v) {
//   echo "x: ", $k, " ", $v, "\n";
// }
