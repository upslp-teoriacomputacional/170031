/* *****************************************************************************
 *  Nombre:     Héctor Hugo Contreras Sánchez
 *  Matrícula:  170031
 *  Carrera:    Tecnologías de la Información
 *  Escuela:    Universidad Politécnica de San Luis Potosí
 *
 *  Materia:    Teoría Computacional
 *  Maestro:    Juan Carlos González Ibarra              
 *                
 *  Descripción: Automatas finitos deterministicos en Rust
 *
 *  Creado:       06/10/2020
 *  Última actualización:  06/10/2020
 *
 * ***************************************************************************** */

// Librerias para poder usar los automatas
use std::process;
use std::io;
use std::str;

//  Definimos la funcion caracter 
fn caracter(character: char) -> i32 {   // La función recibe un caracter y devuelve un entero 
    let mut Fin: char = '\n';           //  Fin es un caracter vacio
    
    //  comparamos si es digito u operador
    if character.is_digit(10){
            return 0;                   //  Si es digito devuelve un cero  
            println!("Ya le mande el 0 crack");
        
    }
    else{                               // Comparamos si es operador
        if character == '+' || character == '-' || character == '*' || character == '/' {
            return 1;                   //  Si es operador devolvemos un dos

            }
        else{                           //  Comparamos si está vacio
			if character == '\n' {
                return 2;               //  Si está vacío devolvemos un uno
            }
        }
        //  Si no es ni un digito ni un operador entonces es un caracter no valido
        println!("Error el caracter: {} no es valido", character);
        process::exit(1);               //Terminamos la ejecución
    }
}
    
    //  Definimos a la funcion  encabezado
    fn encabezado(){
        println!("|  Edo. Actual |Caracter |  Simbolo  |Edo. Siguiente |");
        body();                         //  Llamada a la función body
    }
    
    //  Definimos la funcion contenido donde guarda cada valor despues de encontrarlo en un ciclo
    fn contenido(estadosig: i32, character: char, simbolo: &str, estado: i32){
        if character == '\n'{  
            println!("|    {}    |        |   {}   |     {}      |", estadosig, simbolo, estado);
			}else{
                println!("|    {}    |    {}    |   {}   |     {}      |", estadosig, character, simbolo, estado); 
			}
			body();   
    }
    
    //  Solo muestra la linea que se repetira cada vez que la mandemos a llamar
    fn body(){
        println!("+--------------+---------+-----------+---------------+");
    }

//  MAIN
fn main(){
    let mut  tipo: String = "".to_string();  //  Variable tipo para guardar el tipo de caracter
    let mut  Fin: String = "".to_string();      //  Variable Simbolo de tipo cadena
    let mut estado: i32 = 0;                    //  Variable estado de tipo entero
    
    //Este es la tabla de transiciones del automata AFD creado
    let tabla = [[1,69,69], [69,2,69], [3,69,69], [69,69,65]];  //  E = 69, A = 65
      
    println!("+-------------------------------------+
    |    Ingrese una cadena a evaluar:    |
    +-------------------------------------+");
    
    let mut entrada = String::new();             //  Variable cadena de tipo string	
    io::stdin().read_line(&mut entrada);         //  Hacemos lectura por teclado y lo guardamos en cadena
    body();                                     //  Ejecutamos la función body
    encabezado();                               //  Creamos el encabezado de la tabla

    //  Ciclo para recorrer la cadena
    for  character in entrada.chars(){           //  Recorremos cada caracter de la cadena
        let mut estadosig: i32 = estado;        
        
        //  Llamamos al metodo para saber si es un caracter valido y el valor retornado se guarda en charcaracter
        let mut charcaracter: i32 = caracter(character);    //Evaluamos si es operador, digito o vacio
        
        if charcaracter == 0{                   //  Si es digito la variable tipo guarda "Digito"
      	    tipo = "Digito".to_string();
      	}
      	else if charcaracter == 1{              //  Si es operador la variable tipo guarda "Operador"
      	    tipo = "Operador".to_string();
      	}
      	else if charcaracter == 2{              //  Si es vacio la variable tipo guarda "Vacio"
      	    tipo = "Fin".to_string();
          }
          
        //  Guardamos el estado de acuerdo a la tabla de transiciones
        estado = tabla[estado as usize][charcaracter as usize];
    
        //  Si el valor obtenido es una E imprimimos cadena no valida
        if estado == 69{                        //  Si el estado en el que estamos es el E significa que la cadena no es valida
            println!("|    {}    |  {}    |     {}     |     {}      |", estadosig, character, tipo, estado);
            body();
            println!("|              Cadena No Valida                       |
    +----------------------------------------------------+");
            process::exit(1);
        }
        contenido(estadosig, character, &tipo, estado);
    }
  
    //  Al concluir si el estado no es 3 que es el de aceptacion imprimimos cadena no valida    
    if estado != 65{
        println!("|              Cadena No Valida                       |
    +----------------------------------------------------+");
    }else{
        //  Si el estado es 3 es una cadena de aceptacion
        if estado == 65{
            println!("|     {}      |         |Fin Cadena |               |", estado);
            body();
            println!("|                Cadena Valida                       |
        +----------------------------------------------------+");
        }
    }
}