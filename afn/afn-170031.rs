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
 *  Creado:       15/10/2020
 *  Última actualización:  16/10/2020
 *
 * ***************************************************************************** */

// Librerias para poder usar los automatas
use std::process;
use std::io;
use std::str;

//  Definimos la funcion caracter
//  La función caracter evalua cada uno de los caracteres de nuestra cadena de entrada
//  Compara si es una a, una b, o si es cadena vacía
//  Si se ingresa un valor diferente el automata manda error y termina la ejecución del programa
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

fn main(){
    let mut simbolo: String = "".to_string();  //  Variable tipo para guardar el tipo de caracter que se lee de la cadena
    let mut estado: i32 = 0;                   //  Variable estado de tipo entero que guarda el estado en que estamos
    let mut contador: i32 = 0;                 //  Variable contador que sirve para llevar el conteo de la longitud de la cedana recorrida
    let mut estadosig: i32;                    //  Variable de tipo entero para guardar el estado siguiente de acuerdo a nuestra tabla de transiciones

    //  Esta es la tabla de transiciones de nuestro automata para nuestra expresión regular E(r) = a*ba*
    //  q0(inicial), q1, q2, q3(Aceptación), q4(No valido)
    //  En esta sección de las transiciones recibí asesoría de mi compañero Benjamín Loredo Amaya
    let tabla= [[0,1,4],[2,4,3],[2,4,3]];
    println!("+-------------------------------------+
    |    Ingrese una cadena a evaluar:    |
    +-------------------------------------+");
    
    let mut entrada = String::new();             //  Variable cadena de tipo string	
    io::stdin().read_line(&mut entrada);         //  Hacemos lectura por teclado y lo guardamos en cadena
    body();                                      //  Ejecutamos la función body
    encabezado();                                //  Creamos el encabezado de la tabla

    //  Recorremos la cadena de entrada
    for  character in entrada.chars(){
        contador=contador+1;                               //  Con cada iteración incrememtamos el valor de nuestro contador de la cadena                             

        let mut charcaracter= caracter(character);         //  Verificamos que tipo de caracter hay en esta posición

        //  Guardamos en la variable estado el valor de la posición donde se encuentra en la tabla de transiciones, es decir los estados
        estado = tabla[estado as usize][charcaracter as usize]; //  Aquí se obtiene el estado actual
        estadosig = estado + 1;                                 //  Aquí guardamos el estado actual + 1
	 
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
    
        //  Si estado en el que estamos es 4 significa que la entrada no es valida y terminamos la ejecución
        if estado==4{
            println!("|    {}    |  {}    | Error |     {}      |",estadosig,character,estado);
            body();
            println!("|              Cadena No Valida :(                   |
                    +----------------------------------------------------+");
            process::exit(1);
        }

        //  Si el estado en el que estamos es 3(valido) pero además ya termino de evaluar toda la cadena imprimimos cadena valida
        if (estado==3) && contador as usize==entrada.chars().count(){
            println!("|     {}      |         |Fin Cadena |               |",estado);
            body();
            println!("|                Cadena Valida                       |
             +----------------------------------------------------+");
            process::exit(1);

        }

        //Al aún estar recorriendo la cadena y ya haber ingresado b, "a" debe ciclarse, por lo que regresamos para no dejar que finalice la ejecución
        if estado == 3{
            estado = 2
        }
        
        //  Imprimimos los datos de lo que leimos
        contenido(estado,character,&simbolo,estadosig);
    }
}