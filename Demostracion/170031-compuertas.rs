/* *****************************************************************************
 *  Nombre:     Héctor Hugo Contreras Sánchez
 *  Matrícula:  170031
 *  Carrera:    Tecnologías de la Información
 *  Escuela:    Universidad Politécnica de San Luis Potosí
 *
 *  Materia:    Teoría Computacional
 *  Maestro:    Juan Carlos González Ibarra              
 *                
 *  Descripción: Uso de compuertas lógicas en Rust
 *
 *  Creado:       22/09/2020
 *  Última actualización:  22/09/2020
 *
 * ***************************************************************************** */

 fn main(){


    //  Declaramos una lista en Rust con los dos valores booleanos posibles, True y False.
    let booleanos = vec![false,true];
 
    //  Tabla de verdad OR
    println!("x\ty\tx or y");   //  Para usar el tabulador escribimos /t
    println!("{:-<22}", "");    //  Escribimos el caracter "-" 22 veces especificando el caracter después de : y el valor con <22
    
    //  Para realizar la tabla de verdad empleamos dos ciclos for que recorran la lista.
    for x in booleanos.iter() {                 //  x recorre las filas
        for y in booleanos.iter() {             //  y recorre las columnas
            println!("{}\t{}\t{}", x,y,x|y);    //  imprimimos los valores de x, y y su operación OR (|)
        }
    }

    println!();
    
    //  Tabla de verdad AND
    println!("x\ty\tx and y");  //  Para usar el tabulador escribimos /t
    println!("{:-<22}", "");    //  Escribimos el caracter "-" 22 veces especificando el caracter después de : y el valor con <22
    for x in booleanos.iter() {                 //  x recorre las filas
        for y in booleanos.iter() {             //  y recorre las columnas
            println!("{}\t{}\t{}", x, y, x&y);  //  imprimimos los valores de x, y y su operación AND (&)
        }
    }

    println!();

    //  Tabla de verdad NOT 
    println!("x\tnot x");       //  Para usar el tabulador escribimos /t
    println!("{:-<13}", "");    //  Escribimos el caracter "-" 13 veces especificando el caracter después de : y el valor con <13
    for x in booleanos.iter() { //  Recorremos los valores de la lista que es False y True
        println!("{}\t{}", x, !x);              //  Imprimimos el valor de la lista y su negación NOT (!)
    }
    
    println!();

    //  Tabla de verdad ^
    println!("x\ty\tx ^ y");     //  Para usar el tabulador escribimos /t
    println!("{:-<21}", "");    //  Escribimos el caracter "-" 21 veces especificando el caracter después de : y el valor con <21
    for x in booleanos.iter() {                 //  x recorre las filas
        for y in booleanos.iter() {             //  y recorre las columnas
            println!("{}\t{}\t{}", x, y, x^y);  //  imprimimos los valores de x, y y su operación ^ (^)
        }
    }
 }