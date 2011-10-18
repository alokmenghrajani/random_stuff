<?php

/**
 * Checks if perl code is doing the right thing.
 * Converts int to roman numeral, calls perl code, and compares the result.
 */

function n_to_roman($n) {
  $lev = 0;
  $rom = '';
  while ($n>0) {
    $r = $n % 10;
    $rom = n_to_roman_helper($r, $lev) . $rom;
    $lev++;
    $n = floor($n / 10);
  }
  return $rom;
}

function n_to_roman_helper($r, $lev) {
  $map = array(
    0 => array('I', 'V'),
    1 => array('X', 'L'),
    2 => array('C', 'D'),
    3 => array('M', '?'));

  switch ($r) {
    case '1':
    case '2':
    case '3':
      return str_repeat($map[$lev][0], $r);
    case '4':
      return $map[$lev][0].$map[$lev][1];
    case '5':
    case '6':
    case '7':
    case '8':
      return $map[$lev][1].str_repeat($map[$lev][0], $r-5);
      break;
    case '9':
      return $map[$lev][0].$map[$lev+1][0];
      break;
  }
  return;
}

for ($i=1; $i<=3999; $i++) {
  if ($i % 100 == 0) {
    echo ".";
  }
  $s = n_to_roman($i);
  // check that the perl code returns the right value
  $perl = exec('perl roman_numeral.pl '.$s.' 2>/dev/null');
  if ($perl != $i) {
    echo 'FAILED!', "\n";
    echo 'input', "\t", 'perl', "\t", 'expected', "\n";
    echo $s, "\t", $perl, "\t", $i, "\n";
    exit;
  }
}
echo "\n";
echo 'OK!', "\n";

