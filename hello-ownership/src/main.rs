fn main() {

    //principe de Ownership

    //example 1
    let x = 5; // x est le propriétaire de la valeur 5

    // "Copy" de la valeur 5 de x à y qui devient le propriétaire
    // x reste valide car l'adresse mémoire de x est copié vers y
    let y = x;

    // appel par référence de y
    let z = &y; // z est une référence à y

    //example 2 avec les chaines de caractères
    let s1 = String::from("hello");
    let s2 = s1; // "Move" de s1 à s2
}
