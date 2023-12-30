fn main() {
    let (lexeme, lexeme_errors) = toy_lang::Lexeme::from_file("example.toy");

    println!("{:#?}", lexeme);
    if lexeme_errors.len() > 0 {
        println!("Errors found!\n{:#?}", lexeme_errors);
    }

    let tu = toy_lang::TranslationUnit::from_lexeme(&lexeme);

    println!("{:#?}", tu);

    // toy_lang::asadfasf(&tu, "example.1ll");
}
