import render from './lib/render.js';

const circleCount = 1000;

// Float32Array is a type of TypeArray, which is an object that 
// describes an array-like view of a binary data buffer;
// array contains position data representing position and size,
// on the screen, where every three data points correspond to a given circle
// This single stream of data is sent to GPU on render, and thus
// should be as compact as possible, only send what is necessary
// for rendering
const circleData = new Float32Array(3 * circleCount);

// velocity values; keeps this separate from what is sent to GPU
const circlevData = new Float32Array(2 * circleCount);
console.log(circleData);

// Set values to random positions
function init(displayWidth, displayHeight) {

    for (let i = 0, iv = 0; i < circleData.length; i += 3, iv += 2) {

        circleData[i] = displayWidth * Math.random();
        circleData[i + 1] = displayHeight * Math.random();
        circleData[i + 2] = 10;

        circlevData[iv] = Math.random() - 0.5;
        circlevData[iv + 1] = Math.random() - 0.5;
    }
}

function timeStep(displayWidth, displayHeight) {
    for (let i = 0, iv = 0; i < circleData.length; i += 3, iv += 2) {
        circleData[i] += circlevData[iv];
        circleData[i + 1] += circlevData[iv + 1];
        
        if (circleData[i] > displayWidth || circleData[i] < 0) {
            circlevData[iv] = -circlevData[iv];
        }

        if (circleData[i + 1] > displayHeight || circleData[i + 1] < 0) {
            circlevData[iv + 1] = -circlevData[iv + 1];
        }
    }
}

render(circleData, circleCount, init, timeStep);