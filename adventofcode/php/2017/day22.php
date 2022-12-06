<?php
ini_set('memory_limit','500M');

function idx($arr, $el, $default) {
  if (!isset($arr[$el])) {
    return $default;
  }
  return $arr[$el];
}

$pos_x = 0;
$pos_y = 0;
$dir_x = 1;
$dir_y = 0;

//echo substr(md5("hello world"), 0, 5);

$lines = trim(file_get_contents('day22.txt'));
// $lines = "..#
// #..
// ...";
$lines = explode("\n", $lines);
$size = 25;

$pos_x = ($size -1) / 2;
$pos_y = ($size -1) / 2;
$dir_x = 0;
$dir_y = -1;

$a = [];
foreach ($lines as $k => $line) {
  $t = [];
  for ($i=0; $i<strlen($line); $i++) {
    $t[] = $line[$i];
  }
  $a[] = $t;
}

//print_r($a);
//echo $pos_y, " ", $pos_x, "\n";

$count = 0;
for ($i=0; $i<10000000; $i++) {
  if (!isset($a[$pos_y][$pos_x])) {
    $a[$pos_y][$pos_x] = '.';
  }
  if ($a[$pos_y][$pos_x] == '#') {
    // turn right
    if (($dir_x == 0) && ($dir_y == 1)) {
      $dir_x = -1;
      $dir_y = 0;
    } else if (($dir_x == 0) && ($dir_y == -1)) {
      $dir_x = 1;
      $dir_y = 0;
    } else if (($dir_x == 1) && ($dir_y == 0)) {
      $dir_x = 0;
      $dir_y = 1;
    } else if (($dir_x == -1) && ($dir_y == 0)) {
      $dir_x = 0;
      $dir_y = -1;
    }
  } else if ($a[$pos_y][$pos_x] == '.') {
    // turn left
    if (($dir_x == 0) && ($dir_y == 1)) {
      $dir_x = 1;
      $dir_y = 0;
    } else if (($dir_x == 0) && ($dir_y == -1)) {
      $dir_x = -1;
      $dir_y = 0;
    } else if (($dir_x == 1) && ($dir_y == 0)) {
      $dir_x = 0;
      $dir_y = -1;
    } else if (($dir_x == -1) && ($dir_y == 0)) {
      $dir_x = 0;
      $dir_y = 1;
    }
  } else if ($a[$pos_y][$pos_x] == 'f') {
    $dir_x = -$dir_x;
    $dir_y = -$dir_y;
  }

  if ($a[$pos_y][$pos_x] == '.') {
    $a[$pos_y][$pos_x] = 'w';
  } else if ($a[$pos_y][$pos_x] == 'w') {
    $a[$pos_y][$pos_x] = '#';
    $count++;
  } else if ($a[$pos_y][$pos_x] == '#') {
    $a[$pos_y][$pos_x] = 'f';
  } else if ($a[$pos_y][$pos_x] == 'f') {
    $a[$pos_y][$pos_x] = '.';
  }

  $pos_x += $dir_x;
  $pos_y += $dir_y;
}
//print_r($a);

echo $count;
