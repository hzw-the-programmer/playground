<?php
class A {
  public $id;
}

$a = new A;
$a->id = '123';

$strattr = 'id';
echo $a->{$strattr} . "\n";
