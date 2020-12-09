## Teoria de conjuntos
##### Nombre: Héctor Hugo Contreras Sánchez
##### Matrícula: 170031
##### Escuela: Universidad Politécnica de San Luis Potosí
##### Materia: Teoría Computacional
##### Maestro: Juan Carlos González Ibarra
##### Descripción: Expresiones regulares en Rust


## Objetivos
Los objetivos de este programa es implementar la expresión regular "E(r)= a*.b.a*"

## Desarrollo del programa
Para desarrollar el programa importamos las siguientes librerias
```rust
    use std::process;
    use std::io;
    use std::str;
```
Primero creamos un método que evalue si el caracter que estamos leyendo es una "a", "b" ó vacío.
```rust
    fn caracter(character: char) -> i32 {   // La función recibe un caracter y devuelve un entero 
        
        //  Comparamos si es digito u operador
        if character == 'a'{                //  Si el caracter es a regresa un cero 
            return 0;                      
        }else if character == 'b'{          //  Si el caracter es b regresa un uno 
            return 1; 
        }else if character == '\n'{         //  Si el caracter es vacío regresamos un dos                              
            return 2; 
        }else{
            //  Si no es a, b o vacío el automata termina y manda error
            println!("Error el caracter: {} no es valido", character);
            process::exit(1);               //Terminamos la ejecución
        }
    }
```

Declarmos las variables que nos ayudan a conocer la información del estado actual y el caracter que estamos evaluando de la cadena de entrada
```rust
    let mut simbolo: String = "".to_string();  //  Variable tipo para guardar el tipo de caracter que se lee de la cadena
    let mut estado: i32 = 0;                   //  Variable estado de tipo entero que guarda el estado en que estamos
    let mut contador: i32 = 0;                 //  Variable contador que sirve para llevar el conteo de la longitud de la cedana recorrida
    let mut estadosig: i32;                    //  Variable de tipo entero para guardar el estado siguiente de acuerdo a nuestra tabla de transiciones
```

Para las transiciones recibí asesoría de mi compañero Benjamín Loredo Amaya
Aquí declaramos las transiciones de nuestro automata.
```rust
    //  Esta es la tabla de transiciones de nuestro automata para nuestra expresión regular E(r) = a*ba*
    //  q0(inicial), q1, q2, q3(Aceptación), q4(No valido)
    //  En esta sección de las transiciones recibí asesoría de mi compañero Benjamín Loredo Amaya
    let tabla= [[0,1,4],[2,4,3],[2,4,3]];
```
Para recorrer la cadena de entrada
```rust
    for  character in entrada.chars(){}
```

Con cada caracter de la cadena de entrada evaluamos si es una entrada valida
```rust
    let mut charcaracter= caracter(character);         //  Verificamos que tipo de caracter hay en esta posición
```

Asociamos a un string el tipo de simbolo que estamos evaluando en el caracter
```rust
    //  Asignamos una cadena con el tipo de caracter que leimos
        if charcaracter == 0{
      	    simbolo = "a*".to_string();                    //  El simbolo es a* si leimos a
      	}
      	else if charcaracter == 1{
      	    simbolo = "b".to_string();;                    //  El simbolo es b si leimos b
      	}
      	else if charcaracter == 2{
      	    simbolo = "Fin".to_string();;                  //  El simbolo es Fin si leimos vacio
      	}
```

Si la cadena NO es válida debe estar en el estado 4
```rust
    //  Si estado en el que estamos es 4 significa que la entrada no es valida y terminamos la ejecución
        if estado==4{
            println!("|    {}    |  {}    | Error |     {}      |",estadosig,character,estado);
            body();
            println!("|              Cadena No Valida :(                   |
                    +----------------------------------------------------+");
            process::exit(1);
        }
```

Si la cadena es válida debe estar en el estado 3
```rust
        //  Si el estado en el que estamos es 3(valido) pero además ya termino de evaluar toda la cadena imprimimos cadena valida
        if (estado==3) && contador as usize==entrada.chars().count(){
            println!("|     {}      |         |Fin Cadena |               |",estado);
            body();
            println!("|                Cadena Valida                       |
             +----------------------------------------------------+");
            process::exit(1);

        }
```

## Problemas encontrados y soluciones
Una vez que se ingresa la letra b ya está en estado 3, debemos comparar con la longtud de nuestra variable contador para que comprobemos que ya se recorrió toda la cadena y no solamente hasta el caracter b de nuestra cadena de entrada
Observar que se compara con la variable contador
```rust
    //  Si el estado en el que estamos es 3(valido) pero además ya termino de evaluar toda la cadena imprimimos cadena valida
        if (estado==3) && contador as usize==entrada.chars().count(){
            println!("|     {}      |         |Fin Cadena |               |",estado);
            body();
            println!("|                Cadena Valida                       |
             +----------------------------------------------------+");
            process::exit(1);

        }
```

Otro problema se da que después de ingresar b, tenemos a* por lo que debemos asegurar que con cada a ingresada regresemos al estado anterior y poder seguir evaluando los otros caracteres de la cadena
```rust
    //Al aún estar recorriendo la cadena y ya haber ingresado b, "a" debe ciclarse, por lo que regresamos para no dejar que finalice la ejecución
    if estado == 3{
        estado = 2
    }
```
