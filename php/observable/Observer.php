<?php
interface Observer {
    public function update($observable, $obj);
}
