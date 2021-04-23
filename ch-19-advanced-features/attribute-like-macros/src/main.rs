
fn main() {
    // Similar to custom derive macros, but instead of generating code
    // for the derive attribute they allow us to create new attributes

    // They are also more flexible, derive only works for structs and
    // enums, attributes can be applied to other items as well such as functions

    // An example is :-
    // #[route(GET, "/")]
    // fn index()

    // The #[route] attribute would be defined by the framework as a procedural
    // macro 

}
                