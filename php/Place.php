<?php
class Place {
  public $id;
  public $name;
  public $pid;
  public $level;
  public $children;

  function __construct($id, $name, $pid, $level, $children) {
    $this->id = $id;
    $this->name = $name;
    $this->pid = $pid;
    $this->level = $level;
    $this->children = $children;
  }
}

function getPlace($row, $level) {
  $prefix = 'p' . $level;
  $id = $prefix . 'id';
  $name = $prefix . 'name';
  $pid = $prefix . 'pid';
  $level = $prefix . 'level';

  $place = new Place($row[$id], $row[$name], $row[$pid], $row[$level], []);
  return $place;
}

function findPlace($places, $place) {
  foreach ($places as $p) {
    if ($p->id == $place->id) {
      return $p;
    }
  }
  return null;
}

function addPlace(&$places, $place) {
  $fplace = findPlace($places, $place);
  if (is_null($fplace)) {
    $places[] = $place;
    return $place;
  }
  return $fplace;
}
