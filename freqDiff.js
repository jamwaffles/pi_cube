#!/bin/node

// Find the divisor to get the nearest frequency for the required target.

const _ = require('lodash')

// Base clock. RasPi 3 has 250Mhz base clock
// https://raspberrypi.stackexchange.com/a/3444/22777
const base = 250000000

// Target frequency in Hz
const target = 4678360

let lastDiff = base
let diff = 0 

// Lodash because I suck
for(let divisor of _.range(2, 32768, 2)) {
  const result = base / divisor

  lastDiff = diff
  diff = result - target

  if(diff <= 0) {
  	const closest = Math.min(diff, lastDiff)
  	const closestDivisor = closest === diff ? divisor : divisor + 1

  	console.log({ lastDiff, diff, divisor, closestDivisor, result })

  	process.exit(0)
  }
}