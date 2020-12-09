## Teoria de conjuntos
##### Nombre: Héctor Hugo Contreras Sánchez
##### Matrícula: 170031
##### Escuela: Universidad Politécnica de San Luis Potosí
##### Materia: Teoría Computacional
##### Maestro: Juan Carlos González Ibarra
##### Descripción: Palindromos en Rust


## Objetivos
Los objetivos de este programa es validar si una cadena ingresada es un palindromo

## Palindromos
Es importante conocer que un palindromo tiene ciertas caracteristicas que nos ayudaran con el desarrollo del programa.
Una de ellas es que siempre tienen un número impar de caracteres.
El caracter que se encuentra a la mitad de la cadena se le llama espejo palindromico, debido a que los siguientes caracteres son invertidos a la primera mitad, si lo visualizamos como aun automata la primera mitad es con la que se avanza en el automata y se regresa al estado inicial con los caracteres invertidos, siendo ese el estado final.

## Forca de abarcar el problema
Se utilizara una pila que guardará los caracteres hasta antes del espejo palindromico.
Posteriormente como es una pila la forma de sacar los valores es la siguiente "Ultimo en llegar Primero en Salir", esto hará que al almacenar el contenido de la pila en una cadena obtendremos la primera mitad de la cadena de forma invertida.
Si al comparar el contenido de la pila con la segunda mitad de la cadena ingresada son iguales, tenemos un palindromo.
No es un palindromo si la cadena tiene un numero par de caracteres o si es cadena vacia.

## Desarrollo del programa
Para desarrollar el programa importamos las siguientes librerias de entrada por teclado
```rust
    use std::io;
```

Primero creamos la variable que guarda la cadena de entrada, la cadena que guarda el espejo palindromico y la pila.
```rust
    let mut entrada = String::new();                            //  Variable que guarda la entrada por teclado
    let mut espejo;                                             //  Variable que guarda el espejo palindromico
    let mut pila: Vec<char> = Vec::new();                       //  Creamos la pila
```

Leemos la cadena de entrada por el usuario sin contener espacios
```rust
    println!("Ingresa la cadena sin espacios: ");
    io::stdin().read_line(&mut entrada);                        //  Realizamos la entrada de la cadena a evaluar
```

Guardamos la longitud de la cadena ingresada en una variable
```rust
    let mut longitud = entrada.len();                           //  Guardamos la longitud de la cadena
```

Si la cadena de entrada es vacía o par no es un palindromo
```rust
    if longitud == 1 || (longitud % 2 != 0){                    //  Rust guarda la longitud + 1
        println!("\nLa cadena no es un palindromo");            //  Cadena vacia o par no es palindromo
    }
```

De lo contrario sacamos la primera mitad de la cadena con y sin el espejo palindromico y además la segunda mitad de la cadena
```rust
    else{
        let palindromo: &str = &entrada[..(longitud/2)];        //  Solo debemos guardar la mitad de la cadena
        let mitad1: &str = &entrada[..(longitud/2-1)];          //  Primera mitad de la cadena sin espejo
        let mitad2: &str = &entrada[(longitud/2)..(longitud-1)];//  Guardamos el espejo de la cadena
```

Guardamos la primera mitad de la cadena con espejo palindromico en la pila
```rust
    for caracter in palindromo.chars(){                     //  Guarda los datos en la pila
            pila.push(caracter);                                //  Insertamos datos en la pila con push
    }
```

Imprimimos la primera mitad de la cadena y además sacamos el espejo palindromico y lo imprimimos
```rust
        println!("\nPrimera mitad de la cadena: {}", mitad1);   //  Imprimimos la primera mitad de la cadena
        espejo = pila.pop();                                    //  Sacamos el espejo palindromico
        println!("Espejo palindromico {:?}", espejo);           //  Imprimimos el espejo palindromico
```

Sacamos el contenido de la pila y lo almacenamos en un string, el resultado es la primera mitad pero en modo espejo (invertido)
```rust
        let mut com = String::new();                            //  Cadena que almacena la primera mitad en espejo
        while let Some(top) = pila.pop() {                      //  Sacamos los valores de la pila   
            com += &top.to_string();                            //  Lo añadimos a la variable de la cadena espejo
        }
```

Imprimimos la cadena en espejo y la segunda mitad de la cadena ingresada
```rust
        println!("Primera mitad en espejo: {}", com);           //  Imprimimos la primera mitad en espejo
        println!("Segunda mitad de la cadena ingresada: {}", mitad2);   //  Segunda mitad de la cadena ingresada
```

Si las dos cadenas son iguales es un palindromo, de lo contrario no lo es
```rust
        println!("Primera mitad en espejo: {}", com);           //  Imprimimos la primera mitad en espejo
        println!("Segunda mitad de la cadena ingresada: {}", mitad2);   //  Segunda mitad de la cadena ingresada
```

## Problemas encontrados y soluciones
Rust al ingresar cadenas almacena un caracter vacio por la tecla "enter", entonces la longitud de la cadena es +1.
Para analizar si es cadena vacia se compara con 1.
Para saber si es par se cambia a valores impares.
```rust
    if longitud == 1 || (longitud % 2 != 0){                    //  Rust guarda la longitud + 1
        println!("\nLa cadena no es un palindromo");            //  Cadena vacia o par no es palindromo
    }
```
