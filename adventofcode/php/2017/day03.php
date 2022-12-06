<?php
ini_set('memory_limit','500M');

$a = [];
$a[0] = [];
$a[0][0] = 1;
$i = 1;
$pos_x = 0;
$pos_y = 0;
$step = 1;
echo $i, " ", $pos_x, " ", $pos_y, "\n";
$input = 368078;

for ($j=0; $j<368078; $j++) {
  // go right $step
  echo "  going right $step\n";
  for ($k = 0; $k < $step; $k++) {
    $i++;
    $pos_x++;
    echo $i, " ", $pos_x, " ", $pos_y, "\n";
    $sum = 0;
    for ($ii = -1; $ii<=1; $ii++) {
      for ($jj = -1; $jj<=1; $jj++) {
        if (($ii==0) && ($jj==0)) {
          continue;
        }
        if (!isset($a[$pos_x + $ii])) {
          $a[$pos_x + $ii] = [];
        }
        $sum += $a[$pos_x + $ii][$pos_y + $jj];
        if ($sum > 368078) {
          echo "SUM!!!", $sum, "\n";
        }
      }
    }
    $a[$pos_x][$pos_y] = $sum;
  }
  // go up $step
  echo "  going up $step\n";
  for ($k = 0; $k < $step; $k++) {
    $i++;
    $pos_y--;
    echo $i, " ", $pos_x, " ", $pos_y, "\n";
    $sum = 0;
    for ($ii = -1; $ii<=1; $ii++) {
      for ($jj = -1; $jj<=1; $jj++) {
        if (($ii==0) && ($jj==0)) {
          continue;
        }
        if (!isset($a[$pos_x + $ii])) {
          $a[$pos_x + $ii] = [];
        }
        $sum += $a[$pos_x + $ii][$pos_y + $jj];
        if ($sum > 368078) {
          echo "SUM!!!", $sum, "\n";
        }
      }
    }
    $a[$pos_x][$pos_y] = $sum;

  }
  $step++;
  // go left $step
  echo "  going left $step\n";
  for ($k = 0; $k < $step; $k++) {
    $i++;
    $pos_x--;
    echo $i, " ", $pos_x, " ", $pos_y, "\n";
    $sum = 0;
    for ($ii = -1; $ii<=1; $ii++) {
      for ($jj = -1; $jj<=1; $jj++) {
        if (($ii==0) && ($jj==0)) {
          continue;
        }
        if (!isset($a[$pos_x + $ii])) {
          $a[$pos_x + $ii] = [];
        }
        $sum += $a[$pos_x + $ii][$pos_y + $jj];
        if ($sum > 368078) {
          echo "SUM!!!", $sum, "\n";
        }
      }
    }
    $a[$pos_x][$pos_y] = $sum;

  }
  // go down $step
  echo "  going down $step\n";
  for ($k = 0; $k < $step; $k++) {
    $i++;
    $pos_y++;
    echo $i, " ", $pos_x, " ", $pos_y, "\n";
    $sum = 0;
    for ($ii = -1; $ii<=1; $ii++) {
      for ($jj = -1; $jj<=1; $jj++) {
        if (($ii==0) && ($jj==0)) {
          continue;
        }
        if (!isset($a[$pos_x + $ii])) {
          $a[$pos_x + $ii] = [];
        }
        $sum += $a[$pos_x + $ii][$pos_y + $jj];
        if ($sum > 368078) {
          echo "SUM!!!", $sum, "\n";
        }
      }
    }
    $a[$pos_x][$pos_y] = $sum;

  }
  $step++;
}
