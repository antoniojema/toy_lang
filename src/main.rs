fn main() {
    let lexeme = toy_lang::Lexeme::from_file("example.toy");

    println!("{:#?}", lexeme);

    let tu = toy_lang::TranslationUnit::from_lexeme(&lexeme);

    println!("{:#?}", tu);

    toy_lang::asadfasf(&tu, "example.ll");
}
