<?php
ini_set('memory_limit','500M');

function idx($arr, $el, $default) {
  if (!isset($arr[$el])) {
    return $default;
  }
  return $arr[$el];
}

//echo substr(md5("hello world"), 0, 5);

$lines = trim(file_get_contents('day24.txt'));
// $lines = "0/2
// 2/2
// 2/3
// 3/4
// 3/5
// 0/1
// 10/1
// 9/10";
$lines = explode("\n", $lines);

$components = [];
foreach ($lines as $k => $line) {
  $t = explode("/", $line);
  if (count($t) != 2) {
    die("HERE: " . $line);
  }
  $components[] = [$t[0]|0, $t[1]|0];
}

$current = 0;
$used = [];
$maxmax = 0;
$maxmax2 = 0;
echo search($current, 0, 0);

function search($current, $depth, $here) {
  global $components;
  global $used;
  global $maxmax;
  global $maxmax2;

  if ($depth > $maxmax2) {
    $maxmax2 = $depth;
    $maxmax = 0;
  }
  if ($depth == $maxmax2) {
    if ($here > $maxmax) {
      $maxmax = $here;
    }
  }

  echo count($components), ' depth: ', $depth, ' ', $here, ' ', $maxmax, "\n";

  $max = $current;
  foreach ($components as $k => $c) {
    if (idx($used, $k, false)) {
      continue;
    }

    if ($c[0] == $current) {
      $used[$k] = true;
      //echo 'using ', join(":", $c), "\n";
      $t = search($c[1], $depth + 1, $here + $c[0] + $c[1]);
      //echo 'not using ', join(":", $c), "\n";
      $t += $c[0];
      $used[$k] = false;
      if ($t > $max) {
        $max = $t;
      }
    }
    if ($c[1] == $current) {
      $used[$k] = true;
      //echo 'using ', join(":", $c), "\n";
      $t = search($c[0], $depth + 1, $here + $c[0] + $c[1]) + $c[1];
      //echo 'not using ', join(":", $c), "\n";
      $used[$k] = false;
      if ($t > $max) {
        $max = $t;
      }
    }
  }
  return $max;
}
