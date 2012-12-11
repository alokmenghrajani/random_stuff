<?php

/**
 * Random cellular textures, also known as Voronoi diagram.
 *
 * The algorithm is pretty simple: place a few random spots on an image and
 * every pixel will be colored according to the distance to the closest
 * spot.
 *
 * Suggestions if you want to play and improve this code:
 * - use customizable functions to color pixels instead of a linear gradient
 * - take a variable number of neighbours into account, instead of just the
 *   closest
 * - make the computation faster by using BSP trees or similar data structures
 */

// Image generation parameters.
// Play with these and get very different results
//
// width/height:          final image size
// foreground/background: the two colors which will appear in the image
// n_spots:               how many spots to generate.
// min_distance:          pixels < $min_distance of a spot => $foreground
// max_distance:          pixels > $max_distance of all spots => $background

$width = 800;
$height = 450;
$foreground = 0x1b3978;
$background = 0xc0c0c0;
$n_spots = 70;
$min_distance = 5;
$max_distance = 100;

$img = imagecreatetruecolor($width, $height);

// Generate the random spots
$spots = array();
for ($i = 0; $i < $n_spots; $i++) {
  $spots[] = array('x' => mt_rand(0, $width - 1), 'y' => mt_rand(0, $height - 1));
}

// For each pixel, find the closest spot and compute a color value
for ($x = 0; $x < $width; $x++) {
  for ($y = 0; $y < $height; $y++) {
    // Find the closest spot
    $spot = 0;
    $spot_distance = null;
    for ($i = 0; $i < $n_spots; $i++) {
      $dx = $spots[$i]['x'] - $x;
      $dy = $spots[$i]['y'] - $y;
      $d = sqrt($dx*$dx + $dy*$dy);
      if (($spot_distance === null) || ($d < $spot_distance)) {
        $spot = $i;
        $spot_distance = $d;
      }
    }

    // Set the pixel using a linear gradient
    $d = $spot_distance;
    $color = $foreground;
    if ($d <= $min_distance) {
      $c1 = 1;
      $c2 = 0;
    } else if ($d >= $max_distance) {
      $c1 = 0;
      $c2 = 1;
    } else {
      $c2 = ($d - $min_distance) / ($max_distance - $min_distance);
      $c1 = 1 - $c2;
    }

    // c1 is the contribution of $foreground, c2 is the contribution of
    // background
    $color =
      (($c1 * ($foreground & 0xff0000)) & 0xff0000) +  // red
      (($c1 * ($foreground & 0x00ff00)) & 0x00ff00) +  // green
      (($c1 * ($foreground & 0x0000ff)) & 0x0000ff) +  // blue
      (($c2 * ($background & 0xff0000)) & 0xff0000) +  // red
      (($c2 * ($background & 0x00ff00)) & 0x00ff00) +  // green
      (($c2 * ($background & 0x0000ff)) & 0x0000ff);   // blue

    imagesetpixel($img, $x, $y, $color);
  }
}

imagepng($img, 'cellular_texture.png');
