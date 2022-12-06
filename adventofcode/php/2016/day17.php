<?php
ini_set('memory_limit','10000M');
$token = 'vwbaicqe';

function is_open($c) {
  return ($c == 'b') ||
  ($c == 'c') ||
  ($c == 'd') ||
  ($c == 'e') ||
  ($c == 'f');
}

function solve($pos_x, $pos_y, $path) {
  global $token;
  if (($pos_x < 0) || ($pos_x > 3)) {
    return 0;
  }
  if (($pos_y < 0) || ($pos_y > 3)) {
    return 0;
  }
  if (($pos_x == 3) && ($pos_y == 3)) {
    echo strlen($path), "\n";
    return strlen($path);
  }
  // find door state
  $h = md5($token . $path);

  // up
  if (is_open($h[0])) {
    solve($pos_x-1, $pos_y, $path . 'U');
  }
  // down
  if (is_open($h[1])) {
    solve($pos_x+1, $pos_y, $path . 'D');
  }
  // left
  if (is_open($h[2])) {
    solve($pos_x, $pos_y - 1, $path . 'L');
  }
  if (is_open($h[3])) {
    solve($pos_x, $pos_y + 1, $path . 'R');
  }
  return;
}

solve(0, 0, '');

// $lines = trim(file_get_contents('day17.txt'));
// $lines = split("\n", $lines);
//
// foreach ($lines as $k => $v) {
//   echo "x: ", $k, " ", $v, "\n";
// }
