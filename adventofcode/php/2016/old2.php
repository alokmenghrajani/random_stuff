<?php
ini_set('memory_limit','10000M');

$size = 3005290;

function fix($n) {
  global $size;
  return $n % $size;
}

function process() {
  global $size;

  $arr = [];
  for ($i=0; $i<$size; $i++) {
    $arr[$i] = 1;
  }

  $left = $size;
  $i = 0;
  $last = 0;
  while ($left > 1) {
    $to_walk = floor($left / 2);
    $walked = 0;
    $next = $i + $j;
    while($walked<$to_walk) {
      $next = fix($next + 1);
      if ($arr[$next] != 0) {
        $walked++;
      }
    }
    //echo "elf: ", $i, " steals from ", $next, "\n";
    $last = $i;
    //echo $arr[$next], "\n";
    $arr[$i] = $arr[$next];
    $arr[$next] = 0;
    $left--;
    while (idx($arr, ++$i) == 0);
    $i = fix($i);
  }
  echo $last + 1;
}

//process(5);
process();
//$lines = trim(file_get_contents('day19.txt'));
//process($lines);

//$arr = [1,2, 3];
//unset($arr[1]);
//print_r($arr);
//echo count($arr);
