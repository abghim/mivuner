use lrlex::CTLexerBuilder;
use cfgrammar::yacc::YaccKind;


fn main() -> Result<(), ()>{
    CTLexerBuilder::new()
        .lrpar_config(|ctp| {
            ctp
            	.yacckind(YaccKind::Grmtools)
            	.grammar_in_src_dir("calc.y")

                .unwrap()
        })
        .lexer_in_src_dir("calc.l")
        .unwrap()
        .build()
        .unwrap();
    Ok(())
}
