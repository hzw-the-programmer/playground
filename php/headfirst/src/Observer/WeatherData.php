<?php
namespace Hzw\DesignPatterns\Observer;

class WeatherData implements Subject {
    private $observers = [];
    private $temperature;
    private $humidity;
    private $pressure;

    public function registerObserver($observer) {
        $this->observers[] = $observer;
    }

    public function removeObserver($observer) {
        if ($index = array_search($observer, $this->observers) !== false) {
            unset($this->observers[$index]);
        }
    }

    public function notifyObservers() {
        foreach ($this->observers as $observer) {
            $observer->update($this);
        }
    }

    public function setMeasurements(float $temperature, float $humidity, float $pressure) {
        $this->temperature = $temperature;
        $this->humidity = $humidity;
        $this->pressure = $pressure;
        $this->notifyObservers();
    }

    public function getTemperature() {
        return $this->temperature;
    }

    public function getHumidity() {
        return $this->humidity;
    }

    public function getPressure() {
        return $this->pressure;
    }
}
