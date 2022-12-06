<?php
ini_set('memory_limit','500M');

function idx($arr, $el, $default) {
  if (!isset($arr[$el])) {
    return $default;
  }
  return $arr[$el];
}

function transform($input) {
  $a = 'abcdefghijklmnop';
  $b = 'bkgcdefiholnpmja';
  $output = '                ';
  for ($i=0; $i<16; $i++) {
    $d = strpos($b, $a[$i]);
//    echo $i, ' goes to ', $d, "\n";
    $output[$d] = $input[$i];
  }
  return $output;
}

$a = 'abcdefghijklmnop';
for ($i=999999980; $i<1000000000; $i++) {
  if ($a == 'abcdefghijklmnop') {
    echo 'loop! ', $i, "\n";
  }
  $a = transform($a);
}
echo $a, "\n";
die;



//echo substr(md5("hello world"), 0, 5);

$lines = trim(file_get_contents('day16.txt'));
//$lines = "s1,x3/4,pe/b";
$lines = explode(",", $lines);

$N = 16;
$a = [];
$p = 'a';
for ($i=0; $i<$N; $i++) {
  $a[] = $p++;
}

for ($kk = 0; $kk < 1000000000; $kk++) {
foreach ($lines as $k => $line) {
//  echo $line, "\n";
  $matches = [];
  if (preg_match('/^s(\d+)$/', $line, $matches)) {
    array_shift($matches);
    $t = [];
    for ($i=0; $i<$N; $i++) {
      $t[$i] = $a[($i - $matches[0] + $N) % $N];
    }
    $a = $t;
  } else if (preg_match('/^x(\d+)\/(\d+)$/', $line, $matches)) {
    array_shift($matches);
    $t = $a[$matches[0]];
    $a[$matches[0]] = $a[$matches[1]];
    $a[$matches[1]] = $t;
  } else if (preg_match('/^p([a-z]+)\/([a-z]+)$/', $line, $matches)) {
    array_shift($matches);
    $t = join('', $a);
    $offset1 = strpos($t, $matches[0]);
    $offset2 = strpos($t, $matches[1]);

    $t = $a[$offset1];
    $a[$offset1] = $a[$offset2];
    $a[$offset2] = $t;
  } else {
    die($line);
  }
}
}

echo join('', $a);
