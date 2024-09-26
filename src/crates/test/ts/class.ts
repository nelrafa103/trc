class Persona {
  // Propiedades privadas (solo accesibles dentro de la clase)
  private name: string;
  private age: number;

  // Constructor que inicializa las propiedades de la clase
  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }

  // Método público para acceder al nombre
  public getName(): string {
    return this.name;
  }

  // Método público para obtener la edad
  public getAge(): number {
    return this.age;
  }

  // Método público que imprime una presentación de la persona
  public introduce(): void {
    console.log(
      `Hello, my name is ${this.name} and I am ${this.age} years old.`,
    );
  }
}
