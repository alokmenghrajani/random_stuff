<?php

class Tiff {
  protected $data;
  protected $offsets;

  protected function getWord($offset) {
    if ($offset+2 > strlen($this->data)) {
      throw new Exception("not enough data for $offset");
    }
    return idx(unpack('v', substr($this->data, $offset, 2)), 1);
  }

  protected function getDWord($offset) {
    if ($offset+4 > strlen($this->data)) {
      throw new Exception("not enough data for $offset");
    }
    return idx(unpack('V', substr($this->data, $offset, 4)), 1);
  }

  public function __construct($data) {
    $this->data = $data;
    $this->offsets = array();

    // The offset of the first IFD is in the header
    $offset = $this->getDWord(4);
    $this->offsets[] = 4;

    $this->visitIfd($offset);
  }

  public function inject($hidden_data) {
    $len = strlen($hidden_data);

    // Fix all the offsets in the tiff
    $data = $this->data;

    foreach ($this->offsets as $offset) {
      $t = $this->getDWord($offset);
      $t += $len;
      $data = overwrite($data, $offset, pack('V', $t));
    }

    // Output things
    $output = substr($data, 0, 8);
    $output .= $hidden_data;
    $output .= substr($data, 8);
    return $output;
  }

  protected function visitIfd($offset) {
    $len = $this->getWord($offset);

    for ($i=0; $i<$len; $i++) {
      $e = $this->visitDirectoryEntry($offset, $i);
    }

    $next_offset = $this->getDWord($offset + 2 + 12 * $len);
    if ($next_offset != 0) {
      $this->offsets[] = $offset + 2 + 12 * $len;
      $this->visitIfd($next_offset);
    }
  }

  protected function visitDirectoryEntry($offset, $i) {
    $o = $offset + 2 + 12*$i;

    $tag = $this->getWord($o);
    $type = $this->getWord($o+2);

    $size_arr = array(1, 1, 2, 4, 8, 1, 1, 2, 4, 8, 4, 8);
    my_assert(isset($size_arr[$type-1]), "invalid type: $type at $i");

    $size = $size_arr[$type-1];
    $count = $this->getDWord($o+4);

    if ($size * $count > 4) {
      $is_offset = true;
    } else {
      $is_offset = false;
    }

    if ($tag == 273) {
      // we need to visit the stripOffsets
      my_assert($type == 4, "stripOffsets doesn't have the expected type: $type");

      $this->offsets[] = $o+8;
      $offset = $this->getDWord($o+8);
      $this->visitStripOffsets($offset, $count);
    }

    if ($is_offset) {
      $this->offsets[] = $o+8;
    }
  }

  protected function visitStripOffsets($offset, $num) {
    for ($i=0; $i<$num; $i++) {
      $this->offsets[] = $offset + $i*4;
    }
  }
}
