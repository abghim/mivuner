
%start input
%avoid_insert "NUMBER"

%%
input -> Result<crate::types::Func, ()>
    : expr { $1 }
    ;

expr -> Result<crate::types::Func, ()>
    : expr '+' mul {
    	Ok($1? + $3?)
    }
    | expr '-' mul {
    	Ok($1? - $3?)
    }
    | mul { $1 }
    ;

mul -> Result<crate::types::Func, ()>
    : mul '*' unary {
    	Ok($1? * $3?)
    }
    | mul '/' unary {
    	Ok($1? / $3?)
    }
    | unary { $1 }
    ;

unary -> Result<crate::types::Func, ()>
    : '-' unary  {
    	Ok(- $2?)
    }
    | power { $1 }
    ;

power -> Result<crate::types::Func, ()>
    : primary '^' func   /* right-associative */ {
    	Ok($1? ^ $3?)
    }
    | func { $1 }
    ;

func -> Result<crate::types::Func, ()>
	: 'IDENT' '(' expr ')' {
		let lex = $1.map_err(|_| ())?;
        let s = $lexer.span_str(lex.span());

		match s {
			"sin" => Ok(crate::types::Func::sin($3?)),
			"cos" => Ok(crate::types::Func::cos($3?)),
			"exp" => Ok(crate::types::Func::exp($3?)),
			"ln" => Ok(crate::types::Func::ln($3?)),
			_ => Err(())
		}

	}
	| primary { $1 }
	;

primary -> Result<crate::types::Func, ()>
    : 'NUMBER' {
    	let lex = $1.map_err(|_| ())?;
        let s = $lexer.span_str(lex.span());
        let value = parse_number(s)?;
        Ok(crate::types::Func::Num(value))
    }

    | 'IDENT' {
    	let lex = $1.map_err(|_| ())?;
        let s = $lexer.span_str(lex.span());
        Ok(
        	if s=="x" { crate::types::Func::Var }
         	else { crate::types::Func::Const(s.to_string()) }
        )
    }

    | '(' expr ')' {
    	$2
    }
    ;
%%

fn parse_number(s: &str) -> Result<f32, ()> {
    match s.parse::<f32>() {
        Ok(val) => Ok(val),
        Err(_) => {
            eprintln!("{s} cannot be represented as an f64");
            Err(())
        }
    }
}
