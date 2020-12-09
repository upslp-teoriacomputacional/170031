/* *****************************************************************************
 *  Nombre:     Héctor Hugo Contreras Sánchez
 *  Matrícula:  170031
 *  Carrera:    Tecnologías de la Información
 *  Escuela:    Universidad Politécnica de San Luis Potosí
 *
 *  Materia:    Teoría Computacional
 *  Maestro:    Juan Carlos González Ibarra              
 *                
 *  Descripción: Expresiones regulares en Rust
 *
 *  Creado:       23/10/2020
 *  Última actualización:  23/10/2020
 *
 * ***************************************************************************** */

//  Librería para poder usar las listas y leer entrada por teclado
use std::collections::HashSet;
use std::io;
//  MAIN
fn main(){
    let mut entrada = String::new();                    //  Variable que guarda la entrada por teclado
    let mut pila: Vec<char> = Vec::new();                 //  Creamos la pila

    io::stdin().read_line(&mut entrada);                //  Realizamos la entrada de la cadena a evaluar
    let mut longitud = entrada.len();                   //  Guardamos la longitud de la cadena
    
    if longitud == 1 || (longitud%2                     //  Rust guarda la longitud + 1
        println!("La cadena no es un palindromo");      //  Cadena vacia o par no es palindromo
    }else{
        
    }

    

    /*
    let mut tokens = HashSet::new();                                    //  Lista que guarda todos los datos obtenidos 

    //  Con el split dividimos la cadena en elementos de una lista, la separación es cada espacio dentro de la cadena
    for word in "int result = 1;".split(" "){                           //  Recorremos cada palabra de la cadena
        //  Aquí identificamos si la palabra es un tipo de dato, en este caso entero, string o boleano
        if  word == "str" || word == "int" || word == "bool" {
            tokens.insert(["DATATYPE", word]);                          //  Insertamos en tokens el tipo de dato
        }

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

            else {
                //  Ahora checamos si la palabra es el operador
                if word == "+" || word == "-" || word == "/" || word == "*" || word == "%" || word == "=" {
                    tokens.insert(["OPERATOR", word]);                  //  Si coindice guarda el operador en el arreglo tokens
                }

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
            }  
        }        
    }
    println!("{:?}", tokens); // Outputs the token array

    */
}