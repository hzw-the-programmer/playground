<?php
require __DIR__ . '/vendor/autoload.php';

use Hzw\DesignPatterns\Observer\WeatherData;
use Hzw\DesignPatterns\Observer\CurrentConditionsDisplay;
use Hzw\DesignPatterns\Observer\StatisticsDisplay;

$wd = new WeatherData();

$ccd = new CurrentConditionsDisplay();
$wd->registerObserver($ccd);
$sd = new StatisticsDisplay();
$wd->registerObserver($sd);

$wd->setMeasurements(80, 65, 30.4);
$wd->setMeasurements(82, 70, 29.2);
$wd->setMeasurements(78, 90, 29.2);
