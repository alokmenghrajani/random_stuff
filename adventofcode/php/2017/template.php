<?php
ini_set('memory_limit','500M');

function idx($arr, $el, $default) {
  if (!isset($arr[$el])) {
    return $default;
  }
  return $arr[$el];
}

//echo substr(md5("hello world"), 0, 5);

$lines = trim(file_get_contents('day04.txt'));
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
