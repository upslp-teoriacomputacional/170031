## Teoria de conjuntos
##### Nombre: Héctor Hugo Contreras Sánchez
##### Matrícula: 170031
##### Escuela: Universidad Politécnica de San Luis Potosí
##### Materia: Teoría Computacional
##### Maestro: Juan Carlos González Ibarra
##### Descripción: Automatas finitos deterministicos en Rust


## Objetivos
Los objetivos de este programa es implementar un automata finito deterministico en lenguaje Rust y poder comprobar sun funcionamiento.

## Desarrollo del programa
Para desarrollar el programa importamos las siguientes librerias
```rust
    use std::process;
    use std::io;
    use std::str;
```
Para declarar un caracter vacio
```rust
    let mut Fin: char = '\n';           //  Fin es un caracter vacio
```

Para conocer si un caracter es digito
```rust
    //  comparamos si es digito u operador
    if character.is_digit(10){
			return 0;                   //  Si es digito devuelve un cero          
    }
```

Crear nuestro automata finito deterministico
```rust
    //Este es la tabla de transiciones del automata AFD creado
    let tabla = [[1,69,69], [69,2,69], [3,69,69], [69,69,65]];  //  E = 69, A = 65
```
Lectura por teclado para crear la cadena w
```rust
    let mut entrada = String::new();             //  Variable cadena de tipo string	
    io::stdin().read_line(&mut entrada);         //  Hacemos lectura por teclado y lo guardamos en cadena
```

## Problemas encontrados y soluciones
Uno de los principales problemas fue al querer comprender el automafa codificado en Rust, es importante tener un diagrama de como funciona para evitar problemas.
El automata tiene la posibilidad de leer caracteres pero en este caso se tuvo que usar código ASCCI para una forma más fácil de evaluar.
```rust
    let tabla = [[1,69,69], [69,2,69], [3,69,69], [69,69,65]];  //  E = 69, A = 65
    if estado == 69     //  Si el estado en el que estamos es el E significa que la cadena no es valida
```
