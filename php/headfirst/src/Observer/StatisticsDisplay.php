<?php
namespace Hzw\DesignPatterns\Observer;

class StatisticsDisplay implements DisplayElement, Observer {
    private $temperatures;
    private $humidities;
    private $pressures;
    private $count;

    public function display() {
        echo sprintf("Avg/Max/Min temperature: %.2f/%.2f/%.2f\n",
            $this->temperatures['sum'] / $this->count,
            $this->temperatures['max'], $this->temperatures['min']);
        echo sprintf("Avg/Max/Min humidity: %.2f/%.2f/%.2f\n",
            $this->humidities['sum'] / $this->count,
            $this->humidities['max'], $this->humidities['min']);
        echo sprintf("Avg/Max/Min pressure: %.2f/%.2f/%.2f\n",
            $this->pressures['sum'] / $this->count,
            $this->pressures['max'], $this->pressures['min']);
    }

    public function update($subject) {
        if ($subject instanceof WeatherData) {
            $temperature = $subject->getTemperature();
            $humidity = $subject->getHumidity();
            $pressure = $subject->getPressure();

            if (!$this->count) {
                $this->temperatures['sum'] = $temperature;
                $this->temperatures['max'] = $temperature;
                $this->temperatures['min'] = $temperature;

                $this->humidities['sum'] = $humidity;
                $this->humidities['max'] = $humidity;
                $this->humidities['min'] = $humidity;

                $this->pressures['sum'] = $pressure;
                $this->pressures['max'] = $pressure;
                $this->pressures['min'] = $pressure;
            } else {
                $this->temperatures['sum'] += $temperature;
                $this->temperatures['max'] = max($temperature, $this->temperatures['max']);
                $this->temperatures['min'] = min($temperature, $this->temperatures['min']);

                $this->humidities['sum'] += $humidity;
                $this->humidities['max'] = max($humidity, $this->humidities['max']);
                $this->humidities['min'] = min($humidity, $this->humidities['min']);

                $this->pressures['sum'] += $pressure;
                $this->pressures['max'] = max($pressure, $this->pressures['max']);
                $this->pressures['min'] = min($pressure, $this->pressures['min']);
            }

            $this->count++;

            $this->display();
        }
    }
}
