## Teoria de conjuntos
##### Nombre: Héctor Hugo Contreras Sánchez
##### Matrícula: 170031
##### Escuela: Universidad Politécnica de San Luis Potosí
##### Materia: Teoría Computacional
##### Maestro: Juan Carlos González Ibarra
##### Descripción: Expresiones regulares en Rust


## Objetivos
El desarrollo de este programa permite identificar las distintas secciones que conforman una instrucción de código, funcionando similar a como lo haría un compilador.

## Desarrollo del programa
Para desarrollar el programa importamos las siguiente libreria para el uso de listas
```rust
//  Librería para poder usar las listas
use std::collections::HashSet;
```
Creamos nuestra lista tokens que almacena la información obtenida del análisis.
```rust
    let mut tokens = HashSet::new();                                    //  Lista que guarda todos los datos obtenidos 
```

Dividimos nuestra cadena de entrada en distintos elementos de una lista, la separación es dada por un espacio, por lo que nos divide la cadena en palabras.
```rust
    //  Con el split dividimos la cadena en elementos de una lista, la separación es cada espacio dentro de la cadena
    for word in "int result = 1;".split(" "){                           //  Recorremos cada palabra de la cadena
```

Al estar evaluando cada palabra primero verificamos si es el tipo de dato, para esto comparamos la palabra con los tipos de datos validos.
str, int, bool.
En caso de dar verdadero insertamos la información del datatype a tokens.
```rust
    //  Aquí identificamos si la palabra es un tipo de dato, en este caso entero, string o boleano
        if  word == "str" || word == "int" || word == "bool" {
            tokens.insert(["DATATYPE", word]);                          //  Insertamos en tokens el tipo de dato
        }
```
En caso de no ser el datatype eveluamos si la palabra corresponde al identificador, el identificador es el nombre de la variable, para ello se crea un boleano auxiliar que nos ayudara a conocer si la palabra tiene números.
Para esta sección recibí asesoría de mi compañero Benjamín Loredo Amaya
Recorremos cada caracter de la palabra y si se encuentra un digito manda el auxiliar a false.
Al final si el auxiliar sigue siendo verdadero sabemos que es el identificador y lo guardamos en tokens.
```rust
    //  Si no es el tipo de dato evaluamos si es un identificador (nombre de la variable)
        else{
            //  Para el identificador recibí asesoría de mi compañero Benjamin Loredo
            //  Se verifica que el nombre de la variable no contenga numeros
            //  Se declara una variable auxiliar para conocer si tiene numeros
            let mut identificador: bool = true;                         //  Nuestro auxiliar inicia como verdadero
            for caracter in word.chars(){                               //  Recorremos cada caracter de la palabra
                if !caracter.is_alphabetic(){                           //  Si regresa falso significa que tiene numeros la palabra 
                identificador = false;                                  //  El auxiliar ahora nos indica que no es el identificador
                }
            }
            if identificador {
                tokens.insert(["IDENTIFIER", word]);                    //  Si el auxiliar es verdadero significa que es el identificador
            }

```

Si sigue sien ser el datatype o el identificador evaluamos si la palabra es el operador, para esto solo se compara los strings.
Si es verdadero se inserta la información en tokens.
```rust
    else {
            //  Ahora checamos si la palabra es el operador
            if word == "+" || word == "-" || word == "/" || word == "*" || word == "%" || word == "=" {
                tokens.insert(["OPERATOR", word]);                  //  Si coindice guarda el operador en el arreglo tokens
            }
```

Solo resta comparar si es el valor de la variable o es el fin de instrucción, para esto la palabra la tenemos que dividir en elementos de una nueva lista, la separación en este caso es punto y coma (;), recorremos cada caracter de las nuevas palabras llamadas division y si el auxiliar nos indica que solo son digitos significa que corresponde al valor de la variable, de lo contrario corresponde al fin de la instrucción. 
```rust
// En esta parte evaluamos el punto y coma (END_STATEMENT) y el valor de la variable (INTEGER)
                else{
                    let mut valor: bool = true; 
                    for division in word.split(";"){
                        valor = true;
                        for caracter in division.chars(){               //  Recorremos cada caracter de la palabra
                            if !caracter.is_alphabetic(){               //  Si regresa falso significa que tiene numeros la palabra 
                                valor = false;                          //  El auxiliar ahora nos indica que si es es valor o no
                            }
                        }
                        if valor {
                            tokens.insert(["END_STATEMENT", ";"]);      //  Si el auxiliar es verdadero significa que es el identificador
                        }else{
                            tokens.insert(["INTEGER", division]);
                        }
                    }
                }
```


## Problemas encontrados y soluciones
Rust no permite utilizar índices en sus listas y arreglos, por lo que complicaba identificar si era el fin de instrucción o era el valor de la variable. Otro de los problemas es que no sabemos la longitud del integer, por lo que no se podía saber en que índice terminaba el integer y comenzaba el fin de instrucción.
Para solucionar esta parte se tuvo que separar la palabra en nuevos elementos de una lista donde la separación sea por punto y coma.
Si al evaluar que los nuevos elementos sólo eran digitos correspondían al valor de la variable.
Si contiene un caracter significa qué corresponde al fin de la instrucción.
