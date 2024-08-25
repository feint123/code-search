// Sample class definition
class Person {
  constructor(name, age) {
    this.name = name;
    this.age = age;
  }

  greet() {
    console.log(
      `Hello, my name is ${this.name} and I'm ${this.age} years old.`,
    );
  }
}

// Sample function definition
function calculateArea(length, width) {
  return length * width;
}

// Sample arrow function
const multiply = (a, b) => a * b;

// Sample async function
async function fetchData(url) {
  try {
    const response = await fetch(url);
    return await response.json();
  } catch (error) {
    console.error("Error fetching data:", error);
  }
}

// Sample object literal
const car = {
  brand: "Toyota",
  model: "Corolla",
  year: 2022,
  start() {
    console.log("Engine started!");
  },
};

// Sample destructuring
const { brand, model } = car;

// Sample spread operator
const numbers = [1, 2, 3];
const moreNumbers = [...numbers, 4, 5];

// Sample template literal
const greeting = `Welcome to ${car.brand} ${car.model}!`;

// Sample if-else statement
if (car.year > 2020) {
  console.log("This is a newer model.");
} else {
  console.log("This is an older model.");
}

// Sample switch statement
switch (car.brand) {
  case "Toyota":
    console.log("It's a Toyota!");
    break;
  case "Honda":
    console.log("It's a Honda!");
    break;
  default:
    console.log("Unknown brand");
}

// Sample for loop
for (let i = 0; i < 5; i++) {
  console.log(`Iteration ${i}`);
}

// Sample while loop
let count = 0;
while (count < 3) {
  console.log(`Count is ${count}`);
  count++;
}
