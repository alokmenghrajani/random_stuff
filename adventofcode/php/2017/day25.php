<?php
ini_set('memory_limit','500M');

function idx($arr, $el, $default) {
  if (!isset($arr[$el])) {
    return $default;
  }
  return $arr[$el];
}

$tape = [];
$checksum = 0;
$state = 'a';
$pos = 0;
for ($i=0; $i<12629077; $i++) {
  if ($state == 'a') {
    if (idx($tape, $pos, 0) == 0) {
      $tape[$pos] = 1;
      $checksum++;
      $pos++;
      $state = 'b';
    } else {
      $tape[$pos] = 0;
      $checksum--;
      $pos--;
      $state = 'b';
    }
  } else if ($state == 'b') {
    if (idx($tape, $pos, 0) == 0) {
      $tape[$pos] = 0;
      $pos++;
      $state = 'c';
    } else {
      $tape[$pos] = 1;
      $pos--;
      $state = 'b';
    }
  } else if ($state == 'c') {
    if (idx($tape, $pos, 0) == 0) {
      $tape[$pos] = 1;
      $checksum++;
      $pos++;
      $state = 'd';
    } else {
      $tape[$pos] = 0;
      $checksum--;
      $pos--;
      $state = 'a';
    }
  } else if ($state == 'd') {
    if (idx($tape, $pos, 0) == 0) {
      $tape[$pos] = 1;
      $checksum++;
      $pos--;
      $state = 'e';
    } else {
      $tape[$pos] = 1;
      $pos--;
      $state = 'f';
    }
  } else if ($state == 'e') {
    if (idx($tape, $pos, 0) == 0) {
      $tape[$pos] = 1;
      $checksum++;
      $pos--;
      $state = 'a';
    } else {
      $tape[$pos] = 0;
      $checksum--;
      $pos--;
      $state = 'd';
    }
  } else if ($state == 'f') {
    if (idx($tape, $pos, 0) == 0) {
      $tape[$pos] = 1;
      $checksum++;
      $pos++;
      $state = 'a';
    } else {
      $tape[$pos] = 1;
      $pos--;
      $state = 'e';
    }
  }
}
echo $checksum;

/*


In state F:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state E.
    */
