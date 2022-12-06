<?php
ini_set('memory_limit','500M');

function idx($arr, $el, $default) {
  if (!isset($arr[$el])) {
    return $default;
  }
  return $arr[$el];
}

//echo substr(md5("hello world"), 0, 5);

$lines = file_get_contents('day19.txt');
$lines = explode("\n", $lines);

$pos_x = 1;
$pos_y = 0;
$dir_x = 0;
$dir_y = 1;

$steps = 0;
while (1) {
  echo "steps: ", $steps++;

  $c = $lines[$pos_y][$pos_x];
  echo "\n";
  echo "char: ", $c, "\n";
  echo "x: ", $pos_x, " dx: ", $dir_x, " y :", $pos_y, " dy: ", $dir_y, "\n";
  if (($c == "|") && ($dir_x == 0)) {
    $pos_x = $pos_x + $dir_x;
    $pos_y = $pos_y + $dir_y;
    continue;
  }
  if (($c == "-") && ($dir_y == 0)) {
    $pos_x = $pos_x + $dir_x;
    $pos_y = $pos_y + $dir_y;
    continue;
  }
  if (($c == "|") && ($dir_y == 0)) {
    // skip this
    $pos_x = $pos_x + $dir_x;
    $pos_y = $pos_y + $dir_y;
    continue;
  }
  if (($c == "-") && ($dir_x == 0)) {
    // skip this
    $pos_x = $pos_x + $dir_x;
    $pos_y = $pos_y + $dir_y;
    continue;
  }
  if (($c == '+') && ($dir_x == 0)) {
    if (idx(idx($lines, $pos_y, []), $pos_x + 1, '') == '-') {
      $dir_x = 1;
      $dir_y = 0;
      $pos_x = $pos_x + $dir_x;
      $pos_y = $pos_y + $dir_y;
      continue;
    }
    if (idx(idx($lines, $pos_y, []), $pos_x - 1, '') == '-') {
      $dir_x = -1;
      $dir_y = 0;
      $pos_x = $pos_x + $dir_x;
      $pos_y = $pos_y + $dir_y;
      continue;
    }
    echo $lines[$pos_y];
    die("here 2");
  }
  if (($c == '+') && ($dir_y == 0)) {
    if (idx(idx($lines, $pos_y+1, []), $pos_x, '') == '|') {
      $dir_x = 0;
      $dir_y = 1;
      $pos_x = $pos_x + $dir_x;
      $pos_y = $pos_y + $dir_y;
      continue;
    }
    if (idx(idx($lines, $pos_y-1, []), $pos_x, '') == '|') {
      $dir_x = 0;
      $dir_y = -1;
      $pos_x = $pos_x + $dir_x;
      $pos_y = $pos_y + $dir_y;
      continue;
    }
    echo $lines[$pos_y];
    die("here 2");
  }
  if (preg_match('/^[A-Z]$/', $c)) {
    echo "answer: ", $c, "\n";
    $pos_x = $pos_x + $dir_x;
    $pos_y = $pos_y + $dir_y;
    continue;
  }

  echo $lines[$pos_y];
  die("here 1");
}

die;


$a = [];
foreach ($lines as $k => $line) {
  echo $line, "\n";
  $matches = [];
  if (preg_match('/^(\S+),\s*(\d+)$/', $line, $matches)) {
    array_shift($matches);
    $a[] = $matches;
  }
}

print_r($a);
