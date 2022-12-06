<?php

// $line = '';
// $display = process($line, $display);
// $display = rect($display, 3, 2);
// $display = rotate_column($display, 1, 1);
// $display = rotate_row($display, 0, 4);
// $display = rotate_column($display, 1, 1);
// render($display);

function render($display) {
  $n = 0;
  for ($j=0; $j<6; $j++) {
    for ($i=0; $i<50; $i++) {
      echo ($display[$i][$j] == 0 ? ' ' : '#');
      $n += $display[$i][$j];
    }
    echo "\n";
  }
  echo $n, "\n";
}

function rotate_column($display, $x, $amount) {
  $r = array();
  for ($i=0; $i<50; $i++) {
    for ($j=0; $j<6; $j++) {
      if ($i == $x) {
        $r[$i][$j] = $display[$i][($j - $amount + 6)%6];
      } else {
        $r[$i][$j] = $display[$i][$j];
      }
    }
  }
  return $r;
}

function rotate_row($display, $y, $amount) {
  $r = array();
  for ($i=0; $i<50; $i++) {
    for ($j=0; $j<6; $j++) {
      if ($j == $y) {
        $r[$i][$j] = $display[($i - $amount + 50)%50][$j];
      } else {
        $r[$i][$j] = $display[$i][$j];
      }
    }
  }
  return $r;
}

function rect($display, $a, $b) {
  for ($i=0; $i<$a; $i++) {
    for ($j=0; $j<$b; $j++) {
      $display[$i][$j] = 1;
    }
  }
  return $display;
}

$display = [];
for ($i=0; $i<50; $i++) {
  $display[$i] = [];
  for ($j=0; $j<6; $j++) {
    $display[$i][$j] = 0;
  }
}

$fh = fopen('day8.txt', 'r');
while ($line = trim(fgets($fh))) {
   echo $line, "\n";
   if (preg_match('/rect (\d+)x(\d+)/', $line, $match)) {
     $display = rect($display, $match[1], $match[2]);
   } else if (preg_match('/rotate column x=(\d+) by (\d+)/', $line, $match)) {
     $display = rotate_column($display, $match[1], $match[2]);
   } else if (preg_match('/rotate row y=(\d+) by (\d+)/', $line, $match)) {
     $display = rotate_row($display, $match[1], $match[2]);
   } else {
     die("here: " . $line);
   }
}
fclose($fh);
render($display);
