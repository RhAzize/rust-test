fn main() {

    //principe de Ownership


    //example 1
    let x = 5; // x est le propriétaire de la valeur 5

    // transfert de la valeur 5 de x à y qui devient le propriétaire
    // x devient invalide car l'adresse mémoire de x est transférée à y
    let y = x;

    //example 2
 /*   let s1 = String::from("hello");
    let s2 = s1;*/
}
