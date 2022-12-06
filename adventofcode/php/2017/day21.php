<?php
ini_set('memory_limit','5000M');

function idx($arr, $el, $default) {
  if (!isset($arr[$el])) {
    return $default;
  }
  return $arr[$el];
}

//echo substr(md5("hello world"), 0, 5);

$lines = trim(file_get_contents('day21.txt'));
//$lines = "../.# => ##./#../...\n.#./..#/### => #..#/..../..../#..#";
$lines = explode("\n", $lines);

$rules = [];
foreach ($lines as $k => $line) {
  $matches = [];
  if (preg_match('%^([.#]{2})/([.#]{2}) => ([.#]{3})/([.#]{3})/([.#]{3})$%', $line, $matches)) {
    array_shift($matches);
    $input = $matches[0] . $matches[1];
    $output = [$matches[2], $matches[3], $matches[4]];
    $rules[$input] = $output;
    // todo: rotations and flip

    // 12   31  43  24
    // 34   42  21  13

    // 34   42  21  13
    // 12   31  43  24

    // 21   13  34  42
    // 43   24  12  31

    $rules[$input[0].$input[1].$input[2].$input[3]] = $output;
    $rules[$input[2].$input[0].$input[3].$input[1]] = $output;
    $rules[$input[3].$input[2].$input[1].$input[0]] = $output;
    $rules[$input[1].$input[3].$input[0].$input[2]] = $output;

    $rules[$input[2].$input[3].$input[0].$input[1]] = $output;
    $rules[$input[3].$input[1].$input[2].$input[0]] = $output;
    $rules[$input[1].$input[0].$input[3].$input[2]] = $output;
    $rules[$input[0].$input[2].$input[1].$input[3]] = $output;

    $rules[$input[1].$input[0].$input[3].$input[2]] = $output;
    $rules[$input[0].$input[2].$input[1].$input[3]] = $output;
    $rules[$input[2].$input[3].$input[0].$input[1]] = $output;
    $rules[$input[3].$input[1].$input[2].$input[0]] = $output;
  } else if (preg_match('%^([.#]{3})/([.#]{3})/([.#]{3}) => ([.#]{4})/([.#]{4})/([.#]{4})/([.#]{4})$%', $line, $matches)) {
    array_shift($matches);
    $input = $matches[0] . $matches[1] . $matches[2];
    $output = [$matches[3], $matches[4], $matches[5], $matches[6]];
    $rules[$input] = $output;

    // 012   630    876   258
    // 345   741    543   147
    // 678   852    210   036

    // 678   852    210   036
    // 345   741    543   147
    // 012   630    876   258

    // 210   036    678   852
    // 543   147    345   741
    // 876   258    012   630

    $rules[$input[0].$input[1].$input[2].$input[3].$input[4].$input[5].$input[6].$input[7].$input[8]] = $output;
    $rules[$input[6].$input[3].$input[0].$input[7].$input[4].$input[1].$input[8].$input[5].$input[2]] = $output;
    $rules[$input[8].$input[7].$input[6].$input[5].$input[4].$input[3].$input[2].$input[1].$input[0]] = $output;
    $rules[$input[2].$input[5].$input[8].$input[1].$input[4].$input[7].$input[0].$input[3].$input[6]] = $output;

    $rules[$input[6].$input[7].$input[8].$input[3].$input[4].$input[5].$input[0].$input[1].$input[2]] = $output;
    $rules[$input[8].$input[5].$input[2].$input[7].$input[4].$input[1].$input[6].$input[3].$input[0]] = $output;
    $rules[$input[2].$input[1].$input[0].$input[5].$input[4].$input[3].$input[8].$input[7].$input[6]] = $output;
    $rules[$input[0].$input[3].$input[6].$input[1].$input[4].$input[7].$input[2].$input[5].$input[8]] = $output;

    $rules[$input[2].$input[1].$input[0].$input[5].$input[4].$input[3].$input[8].$input[7].$input[6]] = $output;
    $rules[$input[0].$input[3].$input[6].$input[1].$input[4].$input[7].$input[2].$input[5].$input[8]] = $output;
    $rules[$input[6].$input[7].$input[8].$input[3].$input[4].$input[5].$input[0].$input[1].$input[2]] = $output;
    $rules[$input[8].$input[5].$input[2].$input[7].$input[4].$input[1].$input[6].$input[3].$input[0]] = $output;
  } else {
    echo $line, "\n";
    die('here1');
  }
}

$size = 3;
$grid = [".#.",
         "..#",
         "###"];

for ($i=0; $i<18; $i++) {
  if ($size % 2 == 0) {
    $new_grid = [];
    for ($j = 0; $j<$size; $j += 2) {
      for ($k = 0; $k<$size; $k += 2) {
        // convert the 3x3 subgrid at j, k into a string
        $t = $grid[$j][$k].$grid[$j+1][$k].$grid[$j][$k+1].$grid[$j+1][$k+1];
        if (!isset($rules[$t])) {
          echo "not match for rule: " . $t . "\n";
          die('meh');
        }
        $output = $rules[$t];

        for ($jj=0; $jj<3; $jj++) {
          for ($kk=0; $kk<3; $kk++) {
            $new_grid[$j / 2 * 3 + $jj][$k / 2 * 3 + $kk] = $output[$jj][$kk];
          }
        }
      }
    }
    $grid = $new_grid;
    $size = $size / 2 * 3;

  } else {
    $new_grid = [];
    for ($j = 0; $j<$size; $j += 3) {
      for ($k = 0; $k<$size; $k += 3) {
        // convert the 3x3 subgrid at j, k into a string
        $t = $grid[$j][$k].$grid[$j+1][$k].$grid[$j+2][$k].$grid[$j][$k+1].$grid[$j+1][$k+1].$grid[$j+2][$k+1]
        .$grid[$j][$k+2].$grid[$j+1][$k+2].$grid[$j+2][$k+2];
        if (!isset($rules[$t])) {
          echo "not match for rule: " . $t . "\n";
          die('meh');
        }
        $output = $rules[$t];

        for ($jj=0; $jj<4; $jj++) {
          for ($kk=0; $kk<4; $kk++) {
            $new_grid[$j / 3 * 4 + $jj][$k / 3 * 4 + $kk] = $output[$jj][$kk];
          }
        }
      }
    }
    $grid = $new_grid;
    $size = $size / 3 * 4;
  }

  $count = 0;
  for ($j=0; $j<$size; $j++) {
    for ($k=0; $k<$size; $k++) {
      if ($grid[$j][$k] == '#') {
        $count++;
      }
    }
  }
  echo $i, " size: ", $size, " count: ", $count, "\n";
}
