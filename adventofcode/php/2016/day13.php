<?php

function is_wall($x, $y) {
  $t = $x*$x + 3*$x + 2*$x*$y + $y + $y*$y;
  $t += 1358;
  $i = 0;
  $n = 0;
  while ($t > 0) {
    $n += ($t & 1);
    $t = $t >> 1;
  }
  return $n % 2;
}

$maze = [];
$size = 40;
for ($j=0; $j<$size; $j++) {
  $maze[$j] = [];
  for ($i=0; $i<$size; $i++) {
    $maze[$j][$i] = is_wall($i, $j) ? '#' : 100;
  }
}
$maze[1][1] = 1;

// for ($j=0; $j<$size; $j++) {
//   for ($i=0; $i<$size; $i++) {
//     echo str_pad($maze[$j][$i], 6, ' ');
//   }
//   echo "\n";
// }

$found = false;
while (!$found) {
  $found = true;
  for ($j=0; $j<$size; $j++) {
    for ($i=0; $i<$size; $i++) {
      if ($maze[$j][$i] == '#') {
        continue;
      }
      $n = 100;
      if (is_numeric($maze[$j][$i-1])) {
        $n = $maze[$j][$i-1] + 1;
      }
      if (is_numeric($maze[$j][$i+1])) {
        $n = min($n, $maze[$j][$i+1] + 1);
      }
      if (is_numeric($maze[$j+1][$i])) {
        $n = min($n, $maze[$j+1][$i] + 1);
      }
      if (is_numeric($maze[$j-1][$i])) {
        $n = min($n, $maze[$j-1][$i] + 1);
      }
      if ($n < $maze[$j][$i]) {
        $maze[$j][$i] = $n;
        $found = false;
      }
    }
  }
}

$t = 0;
for ($j=0; $j<$size; $j++) {
  for ($i=0; $i<$size; $i++) {
    if (($maze[$j][$i] != '#') && ($maze[$j][$i] <= 51)) {
      $t++;
    }
    echo str_pad($maze[$j][$i], 4, ' ');
  }
  echo "\n";
}

echo "answer: ", $maze[39][31], "\n\n";
echo $t;

//
//
//
// $lines = trim(file_get_contents('day13.txt'));
// $lines = split("\n", $lines);
//
// foreach ($lines as $k => $v) {
//   echo "x: ", $k, " ", $v, "\n";
// }
