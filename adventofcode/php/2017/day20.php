<?php
ini_set('memory_limit','500M');

function idx($arr, $el, $default) {
  if (!isset($arr[$el])) {
    return $default;
  }
  return $arr[$el];
}

//echo substr(md5("hello world"), 0, 5);

$lines = trim(file_get_contents('day20.txt'));
$lines = explode("\n", $lines);

$p = [];
foreach ($lines as $k => $line) {
  // p=<-4897,3080,2133>, v=<-58,-15,-78>, a=<17,-7,0>
  $values = explode(", ", $line);
  $t = [];
  foreach ($values as $value) {
    echo $value, "\n";
    $matches = [];
    if (preg_match('/^.=<([0-9-]+),([0-9-]+),([0-9-]+)>/', $value, $matches)) {
      array_shift($matches);
      $t[] = $matches;
    } else {
      echo $line, "\n", $value, "\n";
      die('here1');
    }
  }
  $p[] = $t;
}

$kill = [];
for ($time=0; $time<100000; $time++) {
  // update all the particules
  $min = 99999999999;
  $min_index = -1;
  $pos = [];
  for ($i=0; $i<count($p); $i++) {
    //echo "debug: ", $i, "\n";

    // velocity
    $p[$i][1][0] += $p[$i][2][0];
    $p[$i][1][1] += $p[$i][2][1];
    $p[$i][1][2] += $p[$i][2][2];

    // position
    $p[$i][0][0] += $p[$i][1][0];
    $p[$i][0][1] += $p[$i][1][1];
    $p[$i][0][2] += $p[$i][1][2];

    $t = $p[$i][2][0] * $p[$i][2][0] + $p[$i][2][1] * $p[$i][2][1] + $p[$i][2][2] * $p[$i][2][2];
    echo $i, " ", "t = ", $t, "\n";
    //echo $t, " ", $p[$i][2][0], " ", $p[$i][2][1], " ", $p[$i][2][2], "\n";
    if ($t < $min) {
      $min = $t;
      $min_index = $i;
    }

    $t = $p[$i][0][0] . ':' . $p[$i][0][1] . ':' . $p[$i][0][2];
    if (isset($pos[$t])) {
      $kill[$pos[$t]] = true;
      $kill[$i] = true;
    }
    $pos[$t] = $i;
  }
  echo count($p) - count($kill), "\n";
  echo $min, " ", $min_index, "\n";
  die;
}
