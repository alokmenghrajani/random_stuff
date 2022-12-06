<?php
ini_set('memory_limit','10000M');

function process($input) {
  $arr = [];
  for ($i=0; $i<$input; $i++) {
    $arr[$i] = [1, $i+1];
  }
  $i = 0;
  while (count($arr) > 1) {
    //echo "count: ", count($arr), "\n";
    $next = ($i + floor(count($arr) / 2)) % count($arr);
    if ($next == $i) {
      $next = ($next + 1) % count($arr);
    }
    //echo 'elf ', $arr[$i][1], ' steals from ', $arr[$next][1], "\n";
//    echo 'i: ', $i, ', next: ', $next, "\n";
//    print_r($arr);
    $arr[$i][0] += $arr[$next][0];
    unset($arr[$next]);
    $n = [];
    $t = 0;
    $ni = 0;
    foreach ($arr as $k => $v) {
      if ($k == $i) {
        $ni = $t;
      }
      $n[$t++] = $v;
    }
    $arr = $n;
    $i = ($ni + 1) % count($arr);
//    print_r($arr);
  }
  print_r($arr);
}

process(5);
process(3005290);
//$lines = trim(file_get_contents('day19.txt'));
//process($lines);

//$arr = [1,2, 3];
//unset($arr[1]);
//print_r($arr);
//echo count($arr);
