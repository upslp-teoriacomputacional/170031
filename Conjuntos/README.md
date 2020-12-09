## Teoria de conjuntos
##### Nombre: Héctor Hugo Contreras Sánchez
##### Matrícula: 170031
##### Escuela: Universidad Politécnica de San Luis Potosí
##### Materia: Teoría Computacional
##### Maestro: Juan Carlos González Ibarra
##### Descripción: Operaciones con conjuntos en lenguaje Rust

## Operaciones con conjuntos en Rust
  <li>1. In this example, we have defined two set variables and we have performed different set operations: union, intersection, difference and symmetric difference.,  </li>
  <li>2. To understand this example, you should have the knowledge of the following Rust programming topics:  </li>
  <li>3. Rust Sets  </li>
  <li>4. Rust Input, Output and Import  </li>

## Objetivos
Los objetivos de este programa es conocer y emplear los métodos que nos ofrece Rust para declarar conjuntos, manipularlos y realizar operaciones entre estos.

## Desarrollo del programa
Para desarrollar el programa primeramente fue conocer el funcionamiento del lenguaje Rust, incluyendo cosas básicas como la impresión de mensajes en pantalla.

Para la impresión de mensajes se utiliza el método ```println!();``` 
para poder hacer uso de los conjuntos se debe utilizar la librería ```std::collections::HashSet```.

Una vez que se conoce la librería es muy sencillo operar los conjuntos ya que nos ofrece los métodos para poderlos operar.
Para declarar conjuntos:
```rust
    let conjunto_a: HashSet<_> = [1, 2, 3, 4, 5].iter().collect();  //  Declaramos el conjunto
```
Para checar si un elemento pertenece a un conjunto:
```rust
    println!("1 in A: {}", conjunto_a.contains(&1));  //  Checamos pertenencia con "contains"
```
Para transformar a conjuntos:
```rust
    let conjunto_a: HashSet<_> = a.iter().collect();  //  Lo convertimos en conjunto por medio de "iter().collect()"
```
Para eliminar un elemento:
```rust
    conjunto_a.remove(&2);  //  Eliminamos el número 2 por medio de "remove"
```
Para limpiar un conjunto:
```rust
    conjunto_a.clear(); //  Lo limpiamos por medio de "clear"
```
Para copiar un conjunto:
```rust
    let conjunto_b: HashSet<_> = conjunto_a.clone();  //  Creamos un conjunto B y copiamos lo del conjunto A por medio de "clone"
```
Para añadir un elemento a un conjunto:
```rust
    conjunto_b.insert(&987);  //  Insertamos un elemento al conjunto por medio de "insert"
```
Para hacer la unión de dos conjuntos:
```rust
    println!("The union between set A and set B = {:?}", conjunto_a.union(&conjunto_b));  //  Union de dos conjuntos con "union"
```
Para hacer la intersección de dos conjuntos:
```rust
    println!("The intersection between set A and set B = {:?}", conjunto_a.intersection(&conjunto_b));  // Usando "intersection"
```
Para hacer la diferencia de dos conjuntos:
```rust
    println!("The difference between set A and set B = {:?}", conjunto_a.difference(&conjunto_b));  //  Usando "difrerence"
```
Para hacer la diferencia simetrica de dos conjuntos:
```rust
    println!("The symmetric_difference between set A and set B = {:?}", conjunto_a.symmetric_difference(&conjunto_b));  
```
Para comprobar si es subconjunto:
```rust
    println!("Set A subset of set B = {:?}", conjunto_a.is_subset(&conjunto_b));  //  Se hace por medio de "is.subset"
```
Para comprobar si es superconjunto:
```rust
    println!("Set A superset of set B = {:?}", conjunto_a.is_superset(&conjunto_b));  //  Se hace por medio de "is.superset"  
```
Para llamar un método se escribe el nombre, sus parametros y cierra con punto y coma:
```rust
    pertenencia();  //  Ejemplo de llamada a una función 
```

## Problemas encontrados y soluciones
Rust para la declaración de sus variables utiliza la palabra reservada let, sin embargo una vez declarada una variable e incluso los conjuntos no pueden sufrir cambios durante el programa, esto debido al enfoque de paradigma funcional que hace uso Rust. Esto genera problemas al querer añadir elementos, quitar elementos, etc. en los conjuntos, para solucionar esto debemos declararlos como mutables, esto por medio de la palabra reservada mut. 
Para llamar un método se escribe el nombre, sus parametros y cierra con punto y coma:
```rust
    let mut conjunto_a: HashSet<_> = [1, 2, 3, 4, 5].iter().collect();  //  Declaramos el conjunto como mutable
```
Otro problema es que en Rust no se puede transformar las cadenas de texto a conjunto directamente, se debe recorrer las letras de la cadena e insertarlas una por una al conjunto por medio de insert.
```rust
    let c = "Hola Mundo";                               //  Declaramos una cadena de texto
    let mut conjunto_c: HashSet<char> = HashSet::new(); //  Creamos un conjunto vacio c
    for i in c.chars(){                                 //  Iteramos cada letra de la cadena de texto
        conjunto_c.insert(i);                           //  Agregamos cada letra de la cadena de texto al conjunto
    }
```