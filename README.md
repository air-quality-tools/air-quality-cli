# Air quality cli

Community tools for checking air quality. 
For Airthings products, but not affiliated with the Airthings company. 

[Releases](https://github.com/air-quality-tools/air-quality-cli/releases) | [Latest release](https://github.com/air-quality-tools/air-quality-cli/releases/tag/0.2.0)

![Screenshot of air quality TUI](https://raw.githubusercontent.com/air-quality-tools/air-quality-cli/master/resources/air-quality-tui.png)

Image: TUI dashboard running on Respberry Pi 3 with data from the Wave Plus sensor

## Features

The cli runs on a Raspberry pi and connects to an Airthings sensor using Bluetooth. 
The sensor values are stored as CSV files on the disk. 
The cli can also start a Text User Interface (TUI). 

**Runner**: Checks and registers the sensor data at fixed time intervals (5 minutes). 
The process stores the sensor data to disk after every interval (as opposed to only once after exiting) so closing the program should not result in missing data. 
If the Bluetooth connection fails or any other failures occures the runner will restart the Bluetooth service and retry up to 3 times. 

**TUI dashboard**: Show the latest registered values. The quality labels follow the ranges specified by Airthings. 
