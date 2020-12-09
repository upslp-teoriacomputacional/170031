## Teoria de conjuntos
##### Nombre: Héctor Hugo Contreras Sánchez
##### Matrícula: 170031
##### Escuela: Universidad Politécnica de San Luis Potosí
##### Materia: Teoría Computacional
##### Maestro: Juan Carlos González Ibarra
##### Descripción: Compuertas en lenguaje Rust


## Objetivos
Los objetivos de este programa es aplicar el uso de compuertas lógicas en lenguaje rust.
Para este programa se desarrollará la tabla de verdad AND, OR, NOT, XOR.

## Desarrollo del programa
Para desarrollar el programa declaramos una lista con los valores True y False, esto nos va a facilitar desarrollar las tablas de verdad.
```rust
    //  Declaramos una lista en Rust con los dos valores booleanos posibles, True y False.
    let booleanos = vec![false,true];
```
Tabla de verdad OR, para realizarla se debe emplear "|".
```rust
    //  Tabla de verdad OR
    println!("x\ty\tx or y");   //  Para usar el tabulador escribimos /t
    println!("{:-<22}", "");    //  Escribimos el caracter "-" 22 veces especificando el caracter después de : y el valor con <22
    //  Para realizar la tabla de verdad empleamos dos ciclos for que recorran la lista.
    for x in booleanos.iter() {                 //  x recorre las filas
        for y in booleanos.iter() {             //  y recorre las columnas
            println!("{}\t{}\t{}", x,y,x|y);    //  imprimimos los valores de x, y y su operación OR (|)
        }
    }
```

Tabla de verdad AND, para realizarla se debe emplear "&".
```rust
    //  Tabla de verdad AND
    println!("x\ty\tx and y");  //  Para usar el tabulador escribimos /t
    println!("{:-<22}", "");    //  Escribimos el caracter "-" 22 veces especificando el caracter después de : y el valor con <22
    for x in booleanos.iter() {                 //  x recorre las filas
        for y in booleanos.iter() {             //  y recorre las columnas
            println!("{}\t{}\t{}", x, y, x&y);  //  imprimimos los valores de x, y y su operación AND (&)
        }
    }
```

Tabla de verdad NOT, para realizarla se debe emplear "!".
```rust
    //  Tabla de verdad NOT 
    println!("x\tnot x");       //  Para usar el tabulador escribimos /t
    println!("{:-<13}", "");    //  Escribimos el caracter "-" 13 veces especificando el caracter después de : y el valor con <13
    for x in booleanos.iter() { //  Recorremos los valores de la lista que es False y True
        println!("{}\t{}", x, !x);              //  Imprimimos el valor de la lista y su negación NOT (!)
    }
```
Tabla de verdad XOR, para realizarla se debe emplear "^".
```rust
    //  Tabla de verdad XOR
    println!("x\ty\tx ^ y");     //  Para usar el tabulador escribimos /t
    println!("{:-<21}", "");    //  Escribimos el caracter "-" 21 veces especificando el caracter después de : y el valor con <21
    for x in booleanos.iter() {                 //  x recorre las filas
        for y in booleanos.iter() {             //  y recorre las columnas
            println!("{}\t{}\t{}", x, y, x^y);  //  imprimimos los valores de x, y y su operación ^ (^)
        }
    }
```

## Problemas encontrados y soluciones
Al momento de declarar la lista en Rust, es importante que para recorrerla se empleé el método .iter() para poder recorrer los elementos.
```rust
    booleanos.iter()
```
Para repetir un caracter n veces se debe encerrar entre corchetes y colocarlo después de ":", después se coloca el número de veces y por último la separación, que en este caso es vacia.
```rust
    println!("{:-<21}", "");    //  Escribimos el caracter "-" 21 veces especificando el caracter después de : y el valor con <21
```