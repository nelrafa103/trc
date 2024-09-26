/* Typescript */
function sumarNumeros(a: number, b: number): number {
  return a + b;
}

let resultado = sumarNumeros(5, 10);

console.log("El resultado es: " + resultado);
while (resultado != 0) {
  resultado--;
}
let objeto = {
  nombre: "Juan",
  edad: 30,
  // Error de sintaxis: coma extra al final del objeto
};

if (objeto.edad > 18) {
  console.log(objeto.nombre + " es mayor de edad");
}
