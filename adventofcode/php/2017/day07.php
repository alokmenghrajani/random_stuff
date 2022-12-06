<?php
ini_set('memory_limit','500M');

function idx($arr, $el, $default) {
  if (!isset($arr[$el])) {
    return $default;
  }
  return $arr[$el];
}

//echo substr(md5("hello world"), 0, 5);

$lines = trim(file_get_contents('day07.txt'));
$lines = explode("\n", $lines);

// $lines = explode("\n", "pbga (66)
// xhth (57)
// ebii (61)
// havc (66)
// ktlj (57)
// fwft (72) -> ktlj, cntj, xhth
// qoyq (66)
// padx (45) -> pbga, havc, qoyq
// tknk (41) -> ugml, padx, fwft
// jptl (61)
// ugml (68) -> gyxo, ebii, jptl
// gyxo (61)
// cntj (57)");

$programs = [];
$weights = [];
foreach ($lines as $k => $line) {
  $matches = [];
  if (preg_match('/^(.+?) \((\d+)\) -> (.*)$/', $line, $matches)) {
    array_shift($matches);
    $weights[$matches[0]] = $matches[1];
    $t = explode(", ", $matches[2]);
    $programs[$matches[0]] = $t;
  } else if (preg_match('/^(.+?) \((\d+)\)$/', $line, $matches)) {
    array_shift($matches);
    $weights[$matches[0]] = $matches[1];
  }
}

// check each program
check('vtzay');
//check('tknk');

function check($c) {
  global $weights, $programs;

  // for each child, check their weight
  if (isset($programs[$c])) {
    echo "here: ", $c, "\n";
    $children = [];
    foreach ($programs[$c] as $child) {
      // get the size of each child
      $t = check($child);
      echo "  ", $child, " totals ", $t, "\n";
      $children[] = $t;
    }
    for ($i=1; $i<count($children); $i++) {
      if ($children[0] != $children[$i]) {
        echo "found it!";
        echo $weights[$c], "\n";
        print_r($programs[$c]);
        print_r($children);
        die;
      }
    }
    return $weights[$c] + $children[0] * count($children);
  } else {
    echo $c, " weights ", $weights[$c], "\n";
    return $weights[$c];
  }
}

// 266

//
// foreach ($lines as $k => $line) {
//   $matches = [];
//   if (preg_match('/^(.+?) \(\d+\) -> (.*)$/', $line, $matches)) {
//     array_shift($matches);
//     $t = explode(", ", $matches[1]);
//     foreach ($t as $p) {
//       //echo $p, "\n";
//       $programs[$p] = false;
//     }
//   } else if (preg_match('/^(.+?) \(\d+\)$/', $line, $matches)) {
//     array_shift($matches);
//   }
// }
//
// foreach ($programs as $p => $_) {
//   echo $p, "\n";
// }
// //print_r($programs);
