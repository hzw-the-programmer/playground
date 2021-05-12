<?php
namespace Hzw\DesignPatterns\Observer;

class CurrentConditionsDisplay implements DisplayElement, Observer {
    private $temperature;
    private $humidity;
    private $pressure;

    public function display() {
        echo sprintf(
            "Current conditions: %.2f F degrees, %.2f%% humidity, %.2f pressure.\n",
            $this->temperature, $this->humidity, $this->pressure
        );
    }

    public function update($subject) {
        if ($subject instanceof WeatherData) {
            $this->temperature = $subject->getTemperature();
            $this->humidity = $subject->getHumidity();
            $this->pressure = $subject->getPressure();
            $this->display();
        }
    }
}
