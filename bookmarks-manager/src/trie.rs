struct Trie<'a> {
    root: &'a Node<'a>
}

#[derive(Clone, Debug, Default)]
struct Node<'a> {
    letter: char,
    next: Vec<&'a Node<'a>>
}