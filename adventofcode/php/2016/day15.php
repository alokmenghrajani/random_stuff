<?php

$disk = [
  [],
  [17, 5], // disk 1
  [19, 8], // disk 2
  [7,1],
  [13,7],
  [5,1],
  [3,0],
  [11, 0],
];

$pos = 0;
$t = 0;
while (true) {
  $found = true;
  for ($j=1; $j<8; $j++) {
    // check disk[$j] at time $t + $j
    //echo $disk[$j][1], " + ", $t, " % ", $disk[$j][0], "\n";
    $offset = ($disk[$j][1] + $t + $j) % $disk[$j][0];
    if ($offset != 0) {
      $found = false;
      break;
    }
  }
  if ($found) {
    echo "found: ", $t, "\n";
  } else {
    //echo "not found\n";
  }
  $t++;
}

// $lines = trim(file_get_contents('day15.txt'));
// $lines = split("\n", $lines);
//
// foreach ($lines as $k => $v) {
//   echo "x: ", $k, " ", $v, "\n";
// }
