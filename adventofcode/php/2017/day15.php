<?php

$gen_a = 591;
$gen_b = 393;

$score = 0;
for ($i=0; $i<5000000; $i++) {
  while (1) {
    $gen_a = ($gen_a * 16807) % 2147483647;
    if ($gen_a % 4 == 0) {
      break;
    }
  }
  while (1) {
    $gen_b = ($gen_b * 48271) % 2147483647;
    if ($gen_b % 8 == 0) {
      break;
    }
  }
  if (($gen_a & 0xffff) == ($gen_b & 0xffff)) {
    $score++;
  }
}
echo $score, "\n";
