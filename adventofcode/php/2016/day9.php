<?php

$input = file_get_contents('day9.txt');

function compute_len($input) {
  if (strlen($input) == 0) {
    return 0;
  }
  if (preg_match('/^\((\d+)x(\d+)\)/', $input, $match)) {
    $input = substr($input, strlen($match[0]));
    $next = substr($input, 0, $match[1]);
    $repeat = $match[2];
    $input = substr($input, $match[1]);
    //return ($repeat * $match[1]) + compute_len($input);
    return $repeat * compute_len($next) + compute_len($input);
  } else {
    if ($input[0] == "\n") {
      return compute_len(substr($input, 1));
    }
    return 1 + compute_len(substr($input, 1));
  }
}

echo compute_len($input);



//
// while (strlen($input) > 0) {
//   if (preg_match('/^\((\d+)x(\d+)\)/', $input, $match)) {
//     // skip len of $match[0]
//     $input = substr($input, strlen($match[0]));
//     $next = substr($input, 0, $match[1]);
//     $repeat = $match[2];
//     for ($i=0; $i<$repeat; $i++) {
//       echo $next;
//     }
//     $input = substr($input, $match[1]);
//   } else {
//     $t = $input[0];
//     if (preg_match('/\s/m', $t)) {
//
//     } else {
//       echo $t;
//     }
//     $input = substr($input, 1);
//   }
// }
//


//
//
// $fh = fopen('day9.txt', 'r');
// while ($line = trim(fgets($fh))) {
//    echo $line, "\n";
// }
// fclose($fh);
