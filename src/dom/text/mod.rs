struct Text {
    content: Content,
}

struct Content {
    t: Type, // Types
    v: &str, // Value
}

emun Type {}
