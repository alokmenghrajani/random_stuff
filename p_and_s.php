<?php

/**
 * We have two numbers m and n (between 2 and 100). Mr.S is told their sum, Mr.P is told their product.
 * The following dialog takes place:
 *
 * Mr.P: I know the product.
 * Mr.S: I know the sum.
 * Mr.P: I however don't know the numbers.
 * Mr.S: I knew you didn't know. I don't know either.
 * Mr.P: Now I know the numbers.
 * Mr.S: Now I know them too.
 *
 * What are the numbers m and n ?
 */

function idx($arr, $e, $default=null) {
  if (isset($arr[$e])) {
    return $arr[$e];
  }
  return $default;
}

$max = 100;

// step 1: find all the products which have 2 or more ways to factor

$products = array();
for ($i=2; $i<=$max; $i++) {
  for ($j=2; $j<=$i; $j++) {
    $t = $i * $j;
    if (!isset($products[$t])) {
      $products[$t] = array();
    }
    $products[$t][] = array($i, $j);
  }
}
foreach ($products as $k => $v) {
  if (count($v) <= 1) {
    unset($products[$k]);
  }
}
echo "Step 1: ", count($products), " possibilities\n";

// step 2
$sums = array();
for ($i=4; $i<=(2*$max); $i++) {
  // check combinations that sum to $i
  $ok = true;
  for ($a=2; $a<=$i/2; $a++) {
    $b = $i-$a;
    $t = $a*$b;
    if (idx($products, $t, 0) < 2) {
      $ok = false;
      break;
    }
  }
  if ($ok) {
    $sums[$i] = true;
  }
}
echo "Step 2: ", count($sums), " possibilities\n";

// step 3
foreach ($products as $k => $v) {
  foreach ($v as $k2 => $arr) {
    $t = $arr[0] + $arr[1];
    if (!isset($sums[$t])) {
      unset($products[$k][$k2]);
    }
  }
}
foreach ($products as $k => $v) {
  if (count($v) != 1) {
    unset($products[$k]);
  }
}
echo "Step 3: ", count($products), " possibilities\n";

// step 4
foreach ($sums as $k => $v) {
  // check combination still exists and is unique
  $sums[$k] = array();
  for ($a=2; $a<=$k/2; $a++) {
    $b = $k-$a;
    $t = $a*$b;
    if (isset($products[$t])) {
      $sums[$k][] = array($a, $b);
    }
  }
  if (count($sums[$k]) != 1) {
    unset($sums[$k]);
  }
}
echo "Step 4: ", count($sums), " possibilities\n";

print_r($sums);

