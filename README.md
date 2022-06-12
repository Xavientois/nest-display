# Nest Display

The Nest thermostat in my home is on a different floor from where I work. This means that the temperature readings might not directly correspond to the temperature in my office. To address this, I put together this small project to monitor the Nest thermostat data and the temperature in my office side by side. The Raspberry Pi reads the conditions in my office from a DHT11 Hygrothermograph and queries the Nest thermostat state from Google's API. It displays both on a two-row LCD display.

## Hardward components

- Raspberry Pi 4B
- LCD1602 Display with PCF8574 Module
- DHT11 Hygrothermograph
- 10kΩ Resistor

## Schematic Diagram

![Diagram](https://user-images.githubusercontent.com/34867186/173248588-9a6be0bc-a24d-4244-8197-aba7d7e750db.png)

