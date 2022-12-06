<?php
ini_set('memory_limit','500M');

function idx($arr, $el, $default) {
  if (!isset($arr[$el])) {
    return $default;
  }
  return $arr[$el];
}

//echo substr(md5("hello world"), 0, 5);

$a = trim(file_get_contents('day09.txt'));
//$a = "{{},{}}";
$n = 0;
$sum = 0;
$garbage = false;
$s2 = 0;
for ($i=0; $i<strlen($a); $i++) {
  if ($garbage) {
    if ($a[$i] == '>') {
      $garbage = false;
    } else if ($a[$i] == '!') {
      $i++;
    } else {
      $s2++;
    }
  } else {
    if ($a[$i] == '{') {
      $n++;
      echo $n, " ";
      $sum += $n;
    } else if ($a[$i] == '}') {
      $n--;
    } else if ($a[$i] == '<') {
      $garbage = true;
    }
  }
}

echo "sum = ", $sum, "\n";
echo "s2 = ", $s2, "\n";


//
//
//
// $lines = explode("\n", $lines);
//
// $a = [];
// foreach ($lines as $k => $line) {
//   echo "x: ", $k, " ", $line, "\n";
//   $matches = [];
//   if (preg_match('/^(\S+),\s*(\d+)$/', $line, $matches)) {
//     array_shift($matches);
//     $a[] = $matches;
//   }
// }
//
// print_r($a);
