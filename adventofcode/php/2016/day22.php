<?php
ini_set('memory_limit','3G');

function can_move($arr, $a, $b, $c, $d) {
  global $max_x, $max_y;
  if (($a < -1) || ($a > $max_x)) {
    throw new Exception("HERE: " . $a);
  }

  if (($arr[$a][$b][0] <= $arr[$c][$d][1]) && ($arr[$a][$b][0] > 0)) {
    return true;
  }
  return false;
}

$max_x = 0;
$max_y = 0;
$arr = [];

function process($input) {
  global $max_x, $max_y;
  $lines = split("\n", $input);
  foreach ($lines as $k => $v) {
    if (preg_match('/node-x(\d+)-y(\d+)\s+\d+T\s+(\d+)T\s+(\d+)T/', $v, $match)) {
      $x = $match[1];
      $y = $match[2];
      if (!isset($arr[$x])) {
        $arr[$x] = [];
      }
      $arr[$x][$y] = [$match[3], $match[4]];
      $max_x = max($max_x, $x);
      $max_y = max($max_y, $y);
    }
  }
  $max_x++; $max_y++;
  $arr[-1] = [];
  $arr[$max_x] = [];
  for ($i=-1; $i<=$max_x; $i++) {
    $arr[$i][-1] = [9999, 0];
    $arr[$i][$max_y] = [9999, 0];
  }
  for ($j=-1; $j<=$max_y; $j++) {
    $arr[-1][$j] = [9999, 0];
    $arr[$max_x][$j] = [9999, 0];
  }
  print_r($arr);

  for ($i=0; $i<$max_x; $i++) {
    for ($j=0; $j<$max_y; $j++) {
      if (($i == 0) && ($j == 0)) {
        echo "E";
        continue;
      }
      if (($i == $max_x-1) && ($j == '0')) {
        echo "S";
        continue;
      }
      if ($arr[$i][$j][0] > 90) {
        echo '!';
      } else {
        echo ($arr[$i][$j][0] == 0) ? '0' : '*';
      }
    }
    echo "\n";
  }
  echo $max_x, " x ", $max_y, " = ", $max_x * $max_y;
  die;
//  solve($max_x-1, 0);
}

$move_queue = [];

function solve($at_x, $at_y) {
  global $arr, $max_x, $max_y, $move_queue;

  echo "in solve: ", $at_x, " ", $at_y, " ", $max_x, " ", $max_y, "\n";
  if (($at_x == 0) && ($at_y == 0)) {
    echo "FOUND!\n";
    return 0;
  }

  // goal is to move $arr[$max][0] to $arr[0][0]
  // find the pairs that can be moved
  $t = 0;
  for ($i=0; $i<$max_x; $i++) {
    for ($j=0; $j<$max_y; $j++) {
      if (can_move($i, $j, $i+1, $j)) {
        echo "can_move: ", $i, ",", $j, " -> ", $i+1, ", ", $j, "\n";
        $t = $arr[$i][$j][0];
        $arr[$i][$j][0] -= $t;
        $arr[$i][$j][1] += $t;
        $arr[$i+1][$j][0] += $t;
        $arr[$i+1][$j][1] -= $t;
        if (($i == $at_x) && ($j == $at_y)) {
          $r = solve($arr, $i+1, $j);
        } else {
          $r = solve($arr, $at_x, $at_y);
        }
        if ($r !== false) {
          return $r + 1;
        }
        $arr[$i][$j][0] += $t;
        $arr[$i][$j][1] -= $t;
        $arr[$i+1][$j][0] -= $t;
        $arr[$i+1][$j][1] += $t;
      }

      if (can_move($i, $j, $i-1, $j)) {
        echo "can_move: ", $i, ",", $j, " -> ", $i-1, ", ", $j, "\n";
        $t = $arr[$i][$j][0];
        $arr[$i][$j][0] -= $t;
        $arr[$i][$j][1] += $t;
        $arr[$i-1][$j][0] += $t;
        $arr[$i-1][$j][1] -= $t;
        if (($i == $at_x) && ($j == $at_y)) {
          $r = solve($arr, $i-1, $j);
        } else {
          $r = solve($arr, $at_x, $at_y);
        }
        if ($r !== false) {
          return $r + 1;
        }
        $arr[$i][$j][0] += $t;
        $arr[$i][$j][1] -= $t;
        $arr[$i-1][$j][0] -= $t;
        $arr[$i-1][$j][1] += $t;
      }

      if (can_move($i, $j, $i, $j+1)) {
        echo "can_move: ", $i, ",", $j, " -> ", $i, ", ", $j+1, "\n";
        $t = $arr[$i][$j][0];
        $arr[$i][$j][0] -= $t;
        $arr[$i][$j][1] += $t;
        $arr[$i][$j+1][0] += $t;
        $arr[$i][$j+1][1] -= $t;
        if (($i == $at_x) && ($j == $at_y)) {
          $r = solve($arr, $i, $j+1);
        } else {
          $r = solve($arr, $at_x, $at_y);
        }
        if ($r !== false) {
          return $r + 1;
        }
        $arr[$i][$j][0] += $t;
        $arr[$i][$j][1] -= $t;
        $arr[$i][$j+1][0] -= $t;
        $arr[$i][$j+1][1] += $t;
      }

      if (can_move($i, $j, $i, $j-1)) {
        echo "can_move: ", $i, ",", $j, " -> ", $i, ", ", $j-1, "\n";
        $t = $arr[$i][$j][0];
        $arr[$i][$j][0] -= $t;
        $arr[$i][$j][1] += $t;
        $arr[$i][$j-1][0] += $t;
        $arr[$i][$j-1][1] -= $t;
        if (($i == $at_x) && ($j == $at_y)) {
          $r = solve($arr, $i, $j-1);
        } else {
          $r = solve($arr, $at_x, $at_y);
        }
        if ($r !== false) {
          return $r + 1;
        }
        $arr[$i][$j][0] += $t;
        $arr[$i][$j][1] -= $t;
        $arr[$i][$j-1][0] -= $t;
        $arr[$i][$j-1][1] += $t;
      }
    }
  }
  echo "NO SOLUTION FOUND...\n";
  return false;
  //     if (can_move($arr, $i, $j, $i-1, $j)) {
  //       echo "debug: ", $i, " ", $j, " -> ", $i-1, " ", $j, "\n";
  //       $t++;
  //     }
  //     if (can_move($arr, $i, $j, $i, $j+1)) {
  //       echo "debug: ", $i, " ", $j, " -> ", $i, " ", $j+1, "\n";
  //       $t++;
  //     }
  //     if (can_move($arr, $i, $j, $i, $j-1)) {
  //       echo "debug: ", $i, " ", $j, " -> ", $i, " ", $j-1, "\n";
  //       $t++;
  //     }
  //   }
  // }
  // echo $t;

  // $t = 0;
  // foreach ($arr as $k1 => $i) {
  //     foreach ($arr as $k2 => $j) {
  //       if ($k1 == $k2) {
  //         continue;
  //       }
  //       if (($i[0] <= $j[1]) && ($i[0] > 0)) {
  //         $t++;
  //       }
  //     }
  // }
  // echo $t;
}

$lines = trim(file_get_contents('day22.txt'));
//$lines = trim(file_get_contents('day22_sample.txt'));
process($lines);
