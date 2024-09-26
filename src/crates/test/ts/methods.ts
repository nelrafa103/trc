const ejemplo: number = 3;
const cadena: string = "Por ejemplo";

/* Number has 6 methods  */
console.log("toExponential()" + ejemplo.toExponential(2)); // fractionDigits
console.log("toFixed()" + ejemplo.toFixed()); // fractionDigits
console.log("toString():" + ejemplo.toString()); // radix
// Start of Valueof
console.log("valueOf():" + ejemplo.valueOf());
console.log("typeof valueOf():" + typeof ejemplo.valueOf());
// End of Valueof
console.log("toPrecision():" + ejemplo.toPrecision()); // precision
console.log("toLocalString():" + ejemplo.toLocaleString()); // locale
//console.log()
Number.length;
console.log(typeof Number.MAX_SAFE_INTEGER);

console.log(Number.MIN_SAFE_INTEGER);

console.log(Number.MIN_VALUE);

console.log(Number.prototype);

// Instance methods:
console.log(cadena.length);
console.log(cadena.charAt(0));
console.log(cadena.endsWith("ejemplo"));
console.log(cadena.concat("de un string"));
console.log(cadena.includes("Por"));
console.log(cadena.replace("e", "a"));
console.log(cadena.search("Por"));
console.log(cadena.slice(0, 3));
console.log(cadena.split(" "));
console.log(cadena.startsWith("Por"));
console.log(cadena.substr(0, 3));
cadena.toLowerCase();
// Class methods

// Boolean
const booleano: boolean = true;
