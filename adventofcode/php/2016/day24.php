<?php
ini_set('memory_limit','3G');

$arr = [];
$targets = [];
$start_x = -1;
$start_y = -1;
$size_y = 0;
$size_x = 0;
function process($input) {
  global $arr, $cells, $start_x, $start_y, $targets, $size_y, $size_x;

  $lines = split("\n", $input);
  foreach ($lines as $k => $v) {
    $arr[$k] = [];
    $size_y = max($size_y, $k);
    for ($i=0; $i<strlen($v); $i++) {
      $size_x = max($size_x, $i);
      if (is_numeric($v[$i])) {
        if ($v[$i] == 0) {
          $start_y = $k;
          $start_x = $i;
        } else {
          $targets[] = [$k, $i];
        }
        $arr[$k][$i] = '.';
      } else {
        $arr[$k][$i] = $v[$i];
      }
    }
  }
}

function solve($start_y, $start_x, $target_y, $target_x) {
  global $arr, $size_x, $size_y;
  $arr[$start_y][$start_x] = 0;
  while ($arr[$target_y][$target_x] == '.') {
    for ($j=0; $j<$size_y; $j++) {
      for ($i=0; $i<$size_x; $i++) {
        if ($arr[$j][$i] == '#') {
          continue;
        }
        $n = 9999999;
        if (is_numeric($arr[$j][$i])) {
          $n = $arr[$j][$i];
        }
        if (is_numeric($arr[$j-1][$i])) {
          $n = min($n, $arr[$j-1][$i] + 1);
        }
        if (is_numeric($arr[$j+1][$i])) {
          $n = min($n, $arr[$j+1][$i] + 1);
        }
        if (is_numeric($arr[$j][$i+1])) {
          $n = min($n, $arr[$j][$i+1] + 1);
        }
        if (is_numeric($arr[$j][$i-1])) {
          $n = min($n, $arr[$j][$i-1] + 1);
        }
        if ($n != 9999999) {
          $arr[$j][$i] = $n;
        }
      }
    }
  }
  return $arr[$target_y][$target_x];
}

$lines = trim(file_get_contents('day24.txt'));
process($lines);
$cpy = $arr;

$from_to = [];
$visited = [];
for ($i=0; $i<count($targets); $i++) {
  $arr = $cpy;
  $t = solve($start_y, $start_x, $targets[$i][0], $targets[$i][1]);
  echo "0 to ", $i+1, ": ", $t, "\n";
  $from_to[] = [0, $i+1, $t];
  $visited[$i+1] = false;
  $visited[0] = true;
}
for ($i=0; $i<count($targets); $i++) {
  for ($j=$i+1; $j<count($targets); $j++) {
    $arr = $cpy;
    $t = solve($targets[$i][0], $targets[$i][1], $targets[$j][0], $targets[$j][1]);
    echo $i+1, " to ", $j+1, ": ", $t, "\n";
    $from_to[] = [$i+1, $j+1, $t];
    $from_to[] = [$j+1, $i+1, $t];
  }
}

function solve2($from_to, $at, $visited, $left) {
  if ($left == 0) {
    // return distance from here to 0.
    return $from_to[$at-1][2];
    //return 0;
  }
  $min = 9999999;
  for ($i=0; $i<count($from_to); $i++) {
    $a = $from_to[$i][0];
    $b = $from_to[$i][1];
    if (($a == $at) && ($visited[$b] == false)) {
      $visited[$b] = true;
      $t = solve2($from_to, $b, $visited, $left-1);
      if ($t !== false) {
        $min = min($min, $t + $from_to[$i][2]);
      }
      $visited[$b] = false;
    }
  }
  if ($min == 9999999) {
    return false;
  }
  return $min;
}

$solution = solve2($from_to, 0, $visited, count($targets));
echo "solution: ", $solution;

//
// $t = solve($targets[0][0], $targets[0][1], $targets[1][0], $targets[1][1]);
// echo $t;
//
// $arr = $cpy;
// $t = solve($targets[0][0], $targets[0][1], $targets[1][0], $targets[1][1]);
// echo $t;
