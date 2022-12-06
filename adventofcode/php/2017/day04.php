<?php
ini_set('memory_limit','500M');

function idx($arr, $el, $default) {
  if (!isset($arr[$el])) {
    return $default;
  }
  return $arr[$el];
}

//echo substr(md5("hello world"), 0, 5);

$lines = trim(file_get_contents('day04.txt'));
$lines = explode("\n", $lines);

$a = [];
$sum = 0;
foreach ($lines as $k => $line) {
  echo "x: ", $k, " ", $line, "\n";
  $t = explode(" ", $line);
  $n1 = count($t);
  $t2 = [];
  for ($i=0; $i<$n1; $i++) {
    $str = $t[$i];
    // sort $str
    $arr = [];
    for ($j=0; $j<strlen($str); $j++) {
      $arr[] = $str[$j];
    }
    sort($arr);
    $str = join("", $arr);
    $t2[$str] = true;
  }
  $n2 = count($t2);
  if ($n1 == $n2) {
    $sum++;
  }
  // $matches = [];
  // if (preg_match('/^(\S+),\s*(\d+)$/', $line, $matches)) {
  //   array_shift($matches);
  //   $a[] = $matches;
  // }
}

echo $sum;
//print_r($a);
