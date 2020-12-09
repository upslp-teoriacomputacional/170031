/* *****************************************************************************
 *  Nombre:     Héctor Hugo Contreras Sánchez
 *  Matrícula:  170031
 *  Carrera:    Tecnologías de la Información
 *  Escuela:    Universidad Politécnica de San Luis Potosí
 *
 *  Materia:    Teoría Computacional
 *  Maestro:    Juan Carlos González Ibarra              
 *                
 *  Descripción: Operaciones con conjuntos en lenguaje Rust
 *
 *  Creado:       13/09/2020
 *  Última actualización:  14/09/2020
 *
 * ***************************************************************************** */

use std::collections::HashSet; //  Librería para poder usar los conjuntos


/* Set Section */
//  Pertenencia de un elemento en un conjunto
fn pertenencia(){
    let mut conjunto_a: HashSet<_> = [1, 2, 3, 4, 5].iter().collect();  //  Declaramos el conjunto
    println!("1 in A: {}", conjunto_a.contains(&1));                    //  Checamos pertenencia con "contains" 
    println!("1 not in A: {}", !conjunto_a.contains(&1));               //  Negación de pertenencia con "!"
    println!("10 in A: {}", conjunto_a.contains(&10));
    println!("10  not in A: {}", !conjunto_a.contains(&10));
}

//  Convertir a un conjunto
fn transformarconj(){
    let a = vec![1, 2, 3];                              //  Creamos una lista
    let conjunto_a: HashSet<_> = a.iter().collect();    //  Lo convertimos en conjunto por medio de "iter().collect()"
    println!("Set A is: {:?}", &conjunto_a);            //  Imprimimos el conjunto

    let b = [1, 2, 3, 4, 5];                            //  Creamos un arreglo
    let conjunto_b: HashSet<_> = b.iter().collect();    //  Lo convertimos en conjunto por medio de "iter().collect()"
    println!("Set B is: {:?}", &conjunto_b);            //  Imprimimos el conjunto

    let c = "Hola Mundo";                               //  Declaramos una cadena de texto
    let mut conjunto_c: HashSet<char> = HashSet::new(); //  Creamos un conjunto vacio c
    for i in c.chars(){                                 //  Iteramos cada letra de la cadena de texto
        conjunto_c.insert(i);                           //  Agregamos cada letra de la cadena de texto al conjunto
    }
    println!("Set C is: {:?}", &conjunto_c);            //  Imprimimos el conjunto
}

//  Eliminar Elemento del conjunto
fn removeItem(){
    let mut conjunto_a: HashSet<_> = [1, 2, 3, 4, 5].iter().collect();      //  Declaramos el conjunto
    conjunto_a.remove(&2);                                                  //  Eliminamos el número 2 por medio de "remove" 
    println!("The set A after to delete number two: {:?}", &conjunto_a);    //  Imprimimos el conjunto resultante
}

//  Limpiar un conjunto
fn clearSet(){
    let mut conjunto_a: HashSet<_> = [1, 2, 3, 4, 5].iter().collect();  //  Declaramos el conjunto
    conjunto_a.clear();                                                 //  Lo limpiamos por medio de "clear"
    println!("The set A clear: {:?}", &conjunto_a);                     //  Imprimimos el conjunto resultante
}

//  Copiar un conjunto
fn copySet(){
    let conjunto_a: HashSet<_> = [1,2,3,4,5].iter().collect();                  //  Declaramos el conjunto A que tiene elementos
    let conjunto_b: HashSet<_> = conjunto_a.clone();                            //  Creamos un conjunto B y copiamos lo del conjunto A por medio de "clone"
    println!("Set A = {:?} compare set B = {:?}", &conjunto_a, &conjunto_b);    //  Comparamos ambos conjuntos
}

//  Agregar elemento a un conjunto
fn addItem(){
    let mut conjunto_b: HashSet<_> = [3, 4, 5, 6, 7].iter().collect();  //  Declaramos el conjunto
    conjunto_b.insert(&987);                                            //  Insertamos un elemento al conjunto por medio de "insert"
    println!("The new set B = {:?}", &conjunto_b);                      //  Imprimimos el conjunto resultante
}

/* Set Operations */
// Unión de dos conjuntos
fn union(){ 
    let conjunto_a: HashSet<_> = [1, 2, 3, 4, 5].iter().collect();                          //  Declaramos el conjunto A                    
    let conjunto_b: HashSet<_> = [3, 4, 5, 6, 7].iter().collect();                          //  Declaramos el conjunto B
    println!("The union between set A and set B = {:?}", conjunto_a.union(&conjunto_b));    //  Imprimimos la union de los conjuntos por medio de "union"
}

//  Intersección entre dos conjuntos
fn interseccion(){
    let conjunto_a: HashSet<_> = [1, 2, 3, 4, 5].iter().collect();                                      //  Declaramos el conjunto A
    let conjunto_b: HashSet<_> = [3, 4, 5, 6, 7].iter().collect();                                      //  Declaramos el conjunto B
    println!("The intersection between set A and set B = {:?}", conjunto_a.intersection(&conjunto_b));  //  Imprimimos la intersección de ambos conjuntos por medio de "intersection"
}

//  Diferencia entre dos conjuntos
fn diferencia(){
    let conjunto_a: HashSet<_> = [1, 2, 3, 4, 5].iter().collect();                                  //  Declaramos el conjunto A
    let conjunto_b: HashSet<_> = [3, 4, 5, 6, 7].iter().collect();                                  //  Declaramos el conjunto B
    println!("The difference between set A and set B = {:?}", conjunto_a.difference(&conjunto_b));  //  Imprimimos la diferencia de A con B por medio de "difference"
}

//  Diferencia simetrica
fn diferenciaSimetrica(){
    let conjunto_a: HashSet<_> = [1, 2, 3, 4, 5].iter().collect();                                                      //  Declaramos conjunto A
    let conjunto_b: HashSet<_> = [3, 4, 5, 6, 7].iter().collect();                                                      //  Declaramos conjunto B
    let conjunto_c: HashSet<_> = [].iter().collect();                                                                   //  Declaramos conjunto C
    println!("The symmetric_difference between set A and set B = {:?}", conjunto_a.symmetric_difference(&conjunto_b));  //  Diferencia simetrica por medio de "symmetric_difference"
    println!("The symmetric_difference between set B and set A = {:?}", conjunto_b.symmetric_difference(&conjunto_a));
    println!("The symmetric_difference between set A and set C = {:?}", conjunto_a.symmetric_difference(&conjunto_c));
    println!("The symmetric_difference between set B and set C = {:?}", conjunto_b.symmetric_difference(&conjunto_c));
}

//  Subconjunto 
fn subconjunto(){
    let conjunto_a: HashSet<_> = [1, 2, 3, 4, 5].iter().collect();                  //  Declaramos el conjunto A      
    let conjunto_b: HashSet<_> = [3, 4, 5, 6, 7].iter().collect();                  //  Declaramos el conjunto B
    println!("Set A subset of set B = {:?}", conjunto_a.is_subset(&conjunto_b));    //  Checamos si A es subconjunto de B por medio de "is_subset"
    println!("Set B subset of set A = {:?}", conjunto_b.is_subset(&conjunto_a));    //  Checamos si B es subconjunto de A por medio de "is_subset"
}

//  Superconjunto
fn superconjunto(){
    let conjunto_a: HashSet<_> = [1, 2, 3, 4, 5].iter().collect();                      //  Declaramos el conjunto A                   
    let conjunto_b: HashSet<_> = [3, 4, 5, 6, 7].iter().collect();                      //  Declaramos el conjunto B
    println!("Set A superset of set B = {:?}", conjunto_a.is_superset(&conjunto_b));    //  Checamos si A es superconjunto de B por medio de "is_superset"
    println!("Set B superset of set A = {:?}", conjunto_b.is_superset(&conjunto_a));    //  Checamos si B es superconjunto de A por medio de "is_superset"
} 

fn main() {

    /* Set Section */

    let conjunto_a: HashSet<_>   = [1, 2, 3, 4, 5].iter().collect();    //  Declaramos el conjunto A
    let conjunto_b: HashSet<_>   = [3, 4, 5, 6, 7].iter().collect();    //  Declaramos el conjunto B
    let conjunto_c: HashSet<i32> = HashSet::new();                      //  Declaramos el conjunto C
    
    println!("The set A is: {:?}", &conjunto_a); //  Impresión del conjunto a
    println!("The set B is: {:?}", &conjunto_b); //  Impresión del conjunto b
    println!("The set C is: {:?}", &conjunto_c); //  Impresión del conjunto c

    
    pertenencia();              //  Pertenencia de un elemento en un conjunto (Line 22)
    transformarconj();          //  Transformación a conjuntos (Line 31)
    removeItem();               //  Eliminar un elemento de un conjunto (Line 49)
    clearSet();                 //  Limpiar un conjunto (Line 56)
    copySet();                  //  Copiar un conjunto (Line 63)
    addItem();                  //  Añadir un elemento a un conjunto (Line 70)

    /* Set Operations */

    union();                    //  Union de dos conjuntos (Line 78)
    interseccion();             //  Intersección de dos conjuntos (Line 85)
    diferencia();               //  Diferencia entre dos conjuntos (Line 92)
    diferenciaSimetrica();      //  Diferencia simetrica entre dos conjuntos (Line 99)
    subconjunto();              //  Comprobar si es subconjunto (Line 110)
    superconjunto();            //  Comprobar si es superconjunto (Line 118)

}