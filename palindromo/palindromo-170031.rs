/* *****************************************************************************
 *  Nombre:     Héctor Hugo Contreras Sánchez
 *  Matrícula:  170031
 *  Carrera:    Tecnologías de la Información
 *  Escuela:    Universidad Politécnica de San Luis Potosí
 *
 *  Materia:    Teoría Computacional
 *  Maestro:    Juan Carlos González Ibarra              
 *      s          
 *  Descripción: Palindromos en Rust
 *
 *  Creado:       06/11/2020
 *  Última actualización:  06/11/2020
 *
 * ***************************************************************************** */

//  Librería para poder leer entrada por teclado
use std::io;
//  MAIN
fn main(){
    let mut entrada = String::new();                            //  Variable que guarda la entrada por teclado
    let mut espejo;                                             //  Variable que guarda el espejo palindromico
    let mut pila: Vec<char> = Vec::new();                       //  Creamos la pila

    println!("Ingresa la cadena sin espacios: ");
    io::stdin().read_line(&mut entrada);                        //  Realizamos la entrada de la cadena a evaluar

    let mut longitud = entrada.len();                           //  Guardamos la longitud de la cadena
    
    if longitud == 1 || (longitud % 2 != 0){                    //  Rust guarda la longitud + 1
        println!("\nLa cadena no es un palindromo");            //  Cadena vacia o par no es palindromo
    }else{
        let palindromo: &str = &entrada[..(longitud/2)];        //  Solo debemos guardar la mitad de la cadena
        let mitad1: &str = &entrada[..(longitud/2-1)];          //  Primera mitad de la cadena sin espejo
        let mitad2: &str = &entrada[(longitud/2)..(longitud-1)];//  Guardamos el espejo de la cadena

        for caracter in palindromo.chars(){                     //  Guarda los datos en la pila
            pila.push(caracter);                                //  Insertamos datos en la pila con push
        }

        println!("\nPrimera mitad de la cadena: {}", mitad1);   //  Imprimimos la primera mitad de la cadena
        espejo = pila.pop();                                    //  Sacamos el espejo palindromico
        println!("Espejo palindromico {:?}", espejo);           //  Imprimimos el espejo palindromico

        let mut com = String::new();                            //  Cadena que almacena la primera mitad en espejo
        while let Some(top) = pila.pop() {                      //  Sacamos los valores de la pila   
            com += &top.to_string();                            //  Lo añadimos a la variable de la cadena espejo
        }

        println!("Primera mitad en espejo: {}", com);           //  Imprimimos la primera mitad en espejo
        println!("Segunda mitad de la cadena ingresada: {}", mitad2);   //  Segunda mitad de la cadena ingresada

        if mitad2.eq(&com){                                     //  Si la cadena en espejo es igual a la segunda mitad de la cadena
            println!("\nLa cadena SI es un palindromo");        //  Decimos que es palindromo
        }else{
            println!("\nLa cadena NO es un palindromo");        //  Si no, decimos que no es un palindromo
        }
    }
}