// read file
var fs = require('fs');
const path = require('path');
var file = path.join(__dirname, 'input.txt');
var input = fs.readFileSync(file, 'utf8');

// split input at empty line into array 
var inputArray = input.split('\r\n');

// create empty array for output
var elf = 0
var calories_per_elf = []
var ncalories = 0
max = 0
for (var i = 0; i < inputArray.length; i++) {
    if  (inputArray[i] == '') {
        elf = elf + 1;
        // append to array
        calories_per_elf.push(ncalories);
        if (ncalories > max) {
            max = ncalories;
        }
        ncalories = 0;
    } else {
        // add calories as integer
        var calories = parseInt(inputArray[i]);
        ncalories += (calories);
    }
}
calories_per_elf.sort().reverse();
var max = calories_per_elf[0];
var sum_max_3 = calories_per_elf[0] + calories_per_elf[1] + calories_per_elf[2];
console.log("Max calories: ", max);
console.log("Max three calories: ", sum_max_3);
