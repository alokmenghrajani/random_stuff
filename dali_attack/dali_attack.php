<?php

/**
 * Takes a tiff and pdf file and builds a "hybrid" file.
 *
 * Very hacky and ugly code...
 *
 * Some assumptions I made:
 * - tiff file is little endian
 * - only deals with StripOffsets encoded with LONG.
 * - TileOffsets aren't supported, I didn't need them when I wrote
 *   this code...
 *
 * More info here: arxiv.org/pdf/1201.0397
 */

require_once("Tiff.php");

function idx($arr, $el, $default=null) {
  if (isset($arr[$el])) {
    return $arr[$el];
  } else {
    return $default;
  }
}

function my_assert($exp, $reason) {
  if (!$exp) {
    throw new Exception("assertion failed: $reason");
  }
}

function overwrite($data, $offset, $replacement_string) {
  $pre = substr($data, 0, $offset);
  $post = substr($data, $offset+strlen($replacement_string));
  return $pre . $replacement_string . $post;
}

$pdf_file = 'input.pdf';
$tiff_file = 'input.tiff';
$output_file = 'output.tiff';

/* ------------------------------------------------- */

echo "Reading $tiff_file\n";
$tiff_data = file_get_contents($tiff_file);
$pdf = fopen($pdf_file, 'r');
$output = fopen($output_file, 'w');

// 'parse' the tiff file
$tiff = new Tiff($tiff_data);

// some random bytes to throw people off
$random = '';
for ($i=0; $i<1000; $i++) {
  if (mt_rand(0, 100) < 25) {
    $random .= "\n";
  } else {
    $random .= chr(mt_rand(0, 255));
  }
}

echo "Reading $pdf_file\n";
// inject the pdf file inside the tiff file
$output = $tiff->inject($random.file_get_contents($pdf_file));

// write the pdf trailer
$output .= "%%EOF";

file_put_contents($output_file, $output);

echo "Wrote: $output_file\n";
