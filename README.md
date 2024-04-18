# Smart Fluid Flow Meter

[![CircleCI](https://dl.circleci.com/status-badge/img/gh/soonick/smart-fluid-flow-meter/tree/master.svg?style=svg)](https://dl.circleci.com/status-badge/redirect/gh/soonick/smart-fluid-flow-meter/tree/master)

Hardware and software for a system that monitors flow of fluids (most likely water) and reports it to a back-end that can emit alerts based on configured preferences.

- [sketch](/sketch/) - The Arduino code that takes care of reading the sensor and posting measurements to the back-end
- [backend](/backend/) - Contains the back end server where measurements will be posted
