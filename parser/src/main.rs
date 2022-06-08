
use std::fs::File;
use std::io::prelude::*;
use std::process::exit;
use std::collections::HashMap;
use lexer::*;

#[derive(PartialEq,Debug)]
enum Typ {
	Number, Bool, String
}
#[derive(Debug)]
struct Ret_variable {
	value: String, 
	typ: Typ,
}


// struct Parser {
// 	programa: String,
// 	tokens: Vec<Token>,
// 	cabecote_tokens: usize,
// }

// impl Parser {
// }


fn program(lexer: &mut Lexer){
	// "main" <block>
	// println!("{}", lexer.tamanho);
	let mut token = lexer.current_token();
	if token.value == "Erro"{eprintln!("Erro 23: Faltando tokens");exit(1);}
	
	if token.typ == "Reserved"{
		if token.value != "main"{
			eprintln!("Erro: Main não foi encontrada na posicao correta");
			exit(1);
		}
	} else {
		eprintln!("Erro: Main não foi encontrada na posicao correta");
		exit(1);
	}

	token = lexer.next_token();
	if token.value == "Erro"{eprintln!("Erro 36: Faltando tokens");exit(1);}

	block(lexer, false);
}

fn block(lexer: &mut Lexer, is_skip: bool){
	// "{" <sentence> "}"
	
	//name, atribuition, type
	let mut hm_variables: HashMap<String, (bool, String)> = HashMap::new(); 

	// hm by types
	// name, value
	let mut hm_numbers: HashMap<String, i128> = HashMap::new(); 
	let mut hm_strings: HashMap<String, String> = HashMap::new();
	let mut hm_bools: HashMap<String, bool> = HashMap::new();

	//Token local
	let mut token = lexer.current_token();
	if token.value == "Erro"{eprintln!("Erro 55: Faltando tokens");exit(1);}

	//variables control
	let mut control = 0;
	
	if token.typ == "Block"{
		if token.value != "{"{
			eprintln!("Erro: Bloco main`{{}}` incorreto = não é `{{`");
			exit(1);
		}
	} else {
		eprintln!("Erro: Bloco main`{{}}` incorreto = faltando `{{`");
		exit(1);
	}

	control+=1;
	token = lexer.next_token();
	if token.value == "Erro"{eprintln!("Erro 72: Faltando tokens");exit(1);}
	
	if token.value == "}" {
		control -=1;
	} else {
		while token.value != "}" {
			if !is_skip {
				sentence(&mut hm_variables, &mut hm_numbers, &mut hm_strings, &mut hm_bools, lexer);
			}
			token = lexer.next_token();
			if token.value == "Erro"{eprintln!("Erro 80: Faltando tokens");exit(1);}
			
			if token.value == "}" {control -=1;}
		}
	}
	
	

	if control != 0 {
		eprintln!("Erro: Bloco main`{{}}` incorreto = faltando `}}`");
		exit(1);
	}
	
}

fn sentence(hm_variables: &mut HashMap<String, (bool, String)>, hm_numbers:  &mut HashMap<String, i128>, hm_strings: &mut HashMap<String, String>, hm_bools: &mut HashMap<String, bool>, lexer: &mut Lexer){
	// ((<atribuition> ";") | <condition>)*
	
	if lexer.current_token().typ == "Create" || lexer.current_token().typ == "Variable" {
		atribuition(hm_variables,hm_numbers,hm_strings,hm_bools, lexer);

		let token = lexer.current_token();
		if token.value == "Erro"{eprintln!("Erro 102: Faltando tokens");exit(1);}
		
		if token.value != ";" {
			eprintln!("Erro: faltando ';' depois de '{}'", lexer.current_previous_token().value);
			exit(1);
		}
	} else if lexer.current_token().value == "print" {
		print(hm_variables,hm_numbers,hm_strings,hm_bools, lexer);

		let token = lexer.next_token();
		if token.value == "Erro"{eprintln!("Erro 122: Faltando tokens");exit(1);}
		
		if token.value != ";" {
			eprintln!("Erro: faltando ';' depois de '{}'", lexer.current_previous_token().value);
			exit(1);
		}
	} else {
		condition(hm_variables,hm_numbers,hm_strings,hm_bools, lexer);
		// println!("Condition");
	}
	
}


fn atribuition(hm_variables: &mut HashMap<String, (bool, String)>, hm_numbers:  &mut HashMap<String, i128>, hm_strings: &mut HashMap<String, String>,hm_bools: &mut HashMap<String, bool>, lexer: &mut Lexer){
	// (<variable> | <create_variable>) "=" <expression>

	let token = lexer.current_token();
	if token.value == "Erro"{eprintln!("Erro 121: Faltando tokens");exit(1);}

	if token.typ == "Variable" {
		 match hm_variables.get_mut(&token.value) {
			Some(tkn) => {
				if tkn.0 == true {
					let typ = tkn.1.clone();
					let mut token_operator = lexer.next_token();
					if token_operator.value == "Erro"{eprintln!("Erro 128: Faltando tokens");exit(1);}
					
					if token_operator.value == "=" {
						token_operator = lexer.next_token();
						if token_operator.value == "Erro"{eprintln!("Erro 132: Faltando tokens");exit(1);}

						let result = expression(hm_variables, hm_numbers, hm_strings, hm_bools, lexer);
						
						let nomequalquer:&str = &*typ;
						match nomequalquer {
							"Bool" => {
								match hm_bools.get_mut(&token.value){
									Some(tkn_bool) => {
										if result.typ == Typ::Bool {
											*tkn_bool = result.value.parse::<bool>().unwrap();
										} else {
											eprintln!("Erro: Tipos incompativeis: {} {} | {:?}", token.value, nomequalquer, result.typ);
											exit(1);
										}
									},
									None => {
										eprintln!("Erro: Variavel '{}' não foi criada", token.value);
										exit(1);
									},
								}
							},
							"Number" => {
								match hm_numbers.get_mut(&token.value){
									Some(tkn_number) => {
										if result.typ == Typ::Number {
											*tkn_number = result.value.parse::<i128>().unwrap();
										} else {
											eprintln!("Erro: Tipos incompativeis: {} {} | {:?}", token.value, nomequalquer, result.typ);
											exit(1);
										}
									},
									None => {
										eprintln!("Erro: Variavel '{}' não foi criada", token.value);
										exit(1);
									},
								}
							},
							"String" => {
								match hm_strings.get_mut(&token.value){
									Some(tkn_string) => {
										if result.typ == Typ::String {
											*tkn_string = result.value;
										} else {
											eprintln!("Erro: Tipos incompativeis: {} {} | {:?}", token.value, nomequalquer, result.typ);
											exit(1);
										}
									},
									None => {
										eprintln!("Erro: Variavel '{}' não foi criada", token.value);
										exit(1);
									},
								}
							},
							_ => {
								eprintln!("Não encontrado o tipo '{}' da variavel '{}'", nomequalquer, token.value);
								exit(1);
							},
						}
					} else {
						eprintln!("Erro: Faltando '=' para atribuicao '{}'", lexer.current_previous_token().value);
						exit(1);
					}
					
				} else {
					eprintln!("Erro: Variavel '{}' tipo 'const' não pode atribuir um valor novamente", token.value);
					exit(1);
				}
			},
			None => {
				eprintln!("Erro: Variavel '{}' não foi criada", token.value);
				exit(1);
			}
		 }
    } else if token.typ == "Create" {
		create_variable(hm_variables,hm_numbers,hm_strings,hm_bools, lexer);
	}

}

fn create_variable(hm_variables: &mut HashMap<String, (bool, String)>, hm_numbers:  &mut HashMap<String, i128>, hm_strings: &mut HashMap<String, String>,hm_bools: &mut HashMap<String, bool>, lexer: &mut Lexer){
	// ("const " | "var ") <variable> ":" <type>
	let mut reatribuition = false;
	let mut name:String = String::new();
	let mut typ:String = String::new();
	
	let mut token = lexer.current_token();
	if token.value == "Erro"{eprintln!("Erro 205: Faltando tokens");exit(1);}

	if token.value == "let"{
		reatribuition = true;
	}
	
	token = lexer.next_token();
	if token.value == "Erro"{eprintln!("Erro 212: Faltando tokens");exit(1);}


	if token.typ == "Variable" {
		name = token.value;
	} else {
		eprintln!("Erro: '{}' não é uma variavel", token.value);
		exit(1);
	}

	match hm_variables.get_mut(&name) {
		Some(tkn) => {
			eprintln!("Erro: '{}' já foi criada", name);
			exit(1);
		},
		None => {},
	}
	
	token = lexer.next_token();
	if token.value == "Erro"{eprintln!("Erro 231: Faltando tokens");exit(1);}

	if token.value == ":" {
		token = lexer.next_token();
		if token.value == "Erro"{eprintln!("Erro 235: Faltando tokens");exit(1);}

		if token.typ == "Type" {
			typ = token.value;
		}else {
			eprintln!("Erro: Tipo não encontrado '{}:{}'", name, token.value);
		exit(1);
		}
		
	} else {
		eprintln!("Erro: Faltando ':' na atribuição '{}'", lexer.current_previous_token().value);
		exit(1);
	}

	hm_variables.insert(name.clone(), (reatribuition, typ.clone()));

	token = lexer.next_token();
	if token.value == "Erro"{eprintln!("Erro 252: Faltando tokens");exit(1);}

	if token.value != "=" {
		eprintln!("Erro: Faltando '=' na atribuição '{}'", name.clone());
		exit(1);
	}

	token = lexer.next_token();
	if token.value == "Erro"{eprintln!("Erro 260: Faltando tokens");exit(1);}
	
	let result = expression(hm_variables,hm_numbers,hm_strings,hm_bools, lexer);
	
	let nomequalquer:&str = &*typ;
	match nomequalquer {
		"Bool" => {
			if result.typ == Typ::Bool {
				hm_bools.insert(name.clone(), result.value.parse::<bool>().unwrap());
			} else {
				eprintln!("Erro: Tipos incompativeis: {} {} | {:?}", name.clone(), nomequalquer, result.typ);
				exit(1);
			}
		},
		"Number" => {
			if result.typ == Typ::Number {
				hm_numbers.insert(name.clone(), result.value.parse::<i128>().unwrap());
			} else {
				eprintln!("Erro: Tipos incompativeis: {} {} | {:?}", name.clone(), nomequalquer, result.typ);
				exit(1);
			}
		},
		"String" => {
			if result.typ == Typ::String{
				hm_strings.insert(name.clone(), result.value);
			} else {
				eprintln!("Erro: Tipos incompativeis: {} {} | {:?}", name.clone(), nomequalquer, result.typ);
				exit(1);
			}
		},
		_ => {
			eprintln!("Erro: Não encontrado o tipo '{}' da variavel '{}'", nomequalquer, name.clone());
			exit(1);
		},
	}

	
	
}

fn print(hm_variables: &mut HashMap<String, (bool, String)>, hm_numbers:  &mut HashMap<String, i128>, hm_strings: &mut HashMap<String, String>, hm_bools: &mut HashMap<String, bool>, lexer: &mut Lexer){
	let token = lexer.next_token();
	if token.value == "Erro"{eprintln!("Erro 328: Faltando tokens");exit(1);}
	let result = expression(hm_variables,hm_numbers,hm_strings,hm_bools, lexer);
	println!("{:?}", result);
}

fn condition(hm_variables: &mut HashMap<String, (bool, String)>, hm_numbers:  &mut HashMap<String, i128>, hm_strings: &mut HashMap<String, String>,hm_bools: &mut HashMap<String, bool>, lexer: &mut Lexer){
	// <condition_loop> | <condition_if>
	
	let token = lexer.current_token();
	if token.value == "Erro"{eprintln!("Erro 345: Faltando tokens");exit(1);}
	
	if token.value == "while" {
		condition_loop(hm_variables,hm_numbers,hm_strings,hm_bools, lexer)
	} else {
		condition_if(hm_variables,hm_numbers,hm_strings,hm_bools, lexer)
	}
}

fn condition_loop(hm_variables: &mut HashMap<String, (bool, String)>, hm_numbers:  &mut HashMap<String, i128>, hm_strings: &mut HashMap<String, String>,hm_bools: &mut HashMap<String, bool>, lexer: &mut Lexer){
	// "while" <expression> <block>
	let mut token = lexer.next_token();
	if token.value == "Erro"{eprintln!("Erro 362: Faltando tokens");exit(1);}
	let cabecote = lexer.cabecote_tokens;
	
	
	loop {
		lexer.define_cabecote(cabecote);
		let result = expression(hm_variables,hm_numbers,hm_strings,hm_bools, lexer);

		if result.typ == Typ::Bool {
	
			token = lexer.next_token();
			if token.value == "Erro"{eprintln!("Erro 369: Faltando tokens");exit(1);}
			if result.value.parse::<bool>().unwrap() == true {
				block(lexer, false);
			} else {
				block(lexer, true);
				break;
			}
		} 
	}
	
}

fn condition_if(hm_variables: &mut HashMap<String, (bool, String)>, hm_numbers:  &mut HashMap<String, i128>, hm_strings: &mut HashMap<String, String>, hm_bools: &mut HashMap<String, bool>, lexer: &mut Lexer){
	// "if" <expression> <block> ("else" <block>)?

	let mut token = lexer.next_token();
	if token.value == "Erro"{eprintln!("Erro 362: Faltando tokens");exit(1);}

	let result = expression(hm_variables,hm_numbers,hm_strings,hm_bools, lexer);
	
	if result.typ == Typ::Bool {
		let mut enabled_else = false;
		token = lexer.next_token();
			if token.value == "Erro"{eprintln!("Erro 369: Faltando tokens");exit(1);}
		if result.value.parse::<bool>().unwrap() == true {
			block(lexer, false);
			enabled_else = true;
		} else {
			block(lexer, true);
		}

		token = lexer.next_token();
		if token.value == "Erro"{eprintln!("Erro 369: Faltando tokens");exit(1);}

		if token.value == "else"{
			token = lexer.next_token();
			if token.value == "Erro"{eprintln!("Erro 369: Faltando tokens");exit(1);}
			block(lexer, enabled_else);
		}
	} 
}

fn expression(hm_variables: &mut HashMap<String, (bool, String)>, hm_numbers:  &mut HashMap<String, i128>, hm_strings: &mut HashMap<String, String>,hm_bools: &mut HashMap<String, bool>, lexer: &mut Lexer) -> Ret_variable {
	// <simple-expression> (<operator_comparation> <simple-expression>)?
	let simple = simple_expression(hm_variables,hm_numbers,hm_strings,hm_bools, lexer);
	//reconhecer se e bool ou nao 
	
	let token = lexer.current_token();
	if token.value == "Erro"{eprintln!("Erro 316: Faltando tokens");exit(1);}

	if token.typ == "Operator" {
		println!("Entrou");
		let simple_second = simple_expression(hm_variables,hm_numbers,hm_strings,hm_bools, lexer);

		let nomequalquer:&str = &*token.value;
		let ret = match nomequalquer {
			">=" => simple.value >= simple_second.value,
			"<=" => simple.value <= simple_second.value,
			"!=" => simple.value != simple_second.value,
			"==" => simple.value == simple_second.value,
			">" => simple.value > simple_second.value,
			"<" => simple.value < simple_second.value,
			_ => false,
		};
		return Ret_variable{value: ret.to_string(), typ: Typ::Bool}
	
	} else {
		return simple
	}
}

fn simple_expression(hm_variables: &mut HashMap<String, (bool, String)>, hm_numbers:  &mut HashMap<String, i128>, hm_strings: &mut HashMap<String, String>,hm_bools: &mut HashMap<String, bool>, lexer: &mut Lexer) -> Ret_variable {
	// <sign>? <term> (<operator_low> <term>)*
	let mut sign_mult = 1;
	
	let mut token = lexer.current_token();
	if token.value == "Erro"{eprintln!("Erro 316: Faltando tokens");exit(1);}
	
	if token.typ == "Operator" {
		if token.value == "+" {
			sign_mult *= 1;
		} else if token.value == "-" {
			sign_mult *= -1;
		}
		token = lexer.next_token();
		if token.value == "Erro"{eprintln!("Erro 325: Faltando tokens");exit(1);}
	}
	
	
	
	//testar o term antes de testar o resto
	let calculate_high = term(hm_variables,hm_numbers,hm_strings,hm_bools, lexer);
	
	if calculate_high.typ == Typ::Number {
		token = lexer.current_token();
		if token.value == "Erro"{eprintln!("Erro 351: Faltando tokens");exit(1);}
		
		let mut calculate = calculate_high.value.parse::<i128>().unwrap() * sign_mult;
		while token.value == "+" || token.value == "-" {
			
			let operator = token.value;
			
			token = lexer.next_token();
			if token.value == "Erro"{eprintln!("Erro 316: Faltando tokens");exit(1);}
			
			
			let calculate_low = term(hm_variables,hm_numbers,hm_strings,hm_bools, lexer);
	
			if operator == "+" { 
				calculate = calculate + calculate_low.value.parse::<i128>().unwrap();
			} else {
				calculate = calculate - calculate_low.value.parse::<i128>().unwrap();
			}

			token = lexer.current_token();
			
			if token.value == "Erro"{eprintln!("Erro 316: Faltando tokens");exit(1);}
		}

		return Ret_variable{value: calculate.to_string(), typ: Typ::Number}
	} else {
		return calculate_high
	}
	
	

	
}

fn term(hm_variables: &mut HashMap<String, (bool, String)>, hm_numbers:  &mut HashMap<String, i128>, hm_strings: &mut HashMap<String, String>,hm_bools: &mut HashMap<String, bool>, lexer: &mut Lexer) -> Ret_variable {
	// <factor> (<operator_high> <factor>)*
	let mut token = lexer.current_token();
	if token.value == "Erro"{eprintln!("Erro 360: Faltando tokens");exit(1);}
	
	let mut operator:String = String::new();
	let mut calculate = 0;
	let mut ret_factor = factor(hm_variables,hm_numbers,hm_strings,hm_bools, lexer);
	
	
	
	if ret_factor.typ == Typ::Number{
		
		calculate = ret_factor.value.parse::<i128>().unwrap();
		
		let mut token_calculate = lexer.current_token();
		if token_calculate.value == "Erro"{eprintln!("Erro 316: Faltando tokens");exit(1);}
		while token_calculate.value == "*" || token_calculate.value == "/" {
			
			operator = token_calculate.value;
			
			token_calculate = lexer.next_token();
			if token_calculate.value == "Erro"{eprintln!("Erro 316: Faltando tokens");exit(1);}
			
			ret_factor = factor(hm_variables,hm_numbers,hm_strings,hm_bools, lexer);
			// println!("{} {:?}", ret_factor.value, ret_factor.typ);
			if ret_factor.typ == Typ::Number{
				if operator == "*" { 
					calculate = calculate * ret_factor.value.parse::<i128>().unwrap();
				} else {
					calculate = calculate / ret_factor.value.parse::<i128>().unwrap();
				}
			} else {
				eprintln!("Erro 398: e uma string ou bool");
				exit(1);
			}
			token_calculate = lexer.current_token();
			if token_calculate.value == "Erro"{eprintln!("Erro 316: Faltando tokens");exit(1);}
		}
		return Ret_variable{value: calculate.to_string(), typ: Typ::Number};
		
	} else {
		return ret_factor;
	}

}

fn factor(hm_variables: &mut HashMap<String, (bool, String)>, hm_numbers:  &mut HashMap<String, i128>, hm_strings: &mut HashMap<String, String>,hm_bools: &mut HashMap<String, bool>, lexer: &mut Lexer) -> Ret_variable{
	// <variable> | <number> | <string> | "(" <expression> ")"
	let mut token = lexer.current_token();
	if token.value == "Erro"{eprintln!("Erro 403: Faltando tokens");exit(1);}

	if token.typ == "Block" && token.value == "(" {
		token = lexer.next_token();
		if token.value == "Erro"{eprintln!("Erro 407: Faltando tokens");exit(1);}
		
		let mut result = expression(hm_variables,hm_numbers,hm_strings,hm_bools, lexer);
		
		token = lexer.current_token();
		if token.value == "Erro"{eprintln!("Erro 412: Faltando tokens");exit(1);}
		
		if token.typ == "Block" && token.value == ")" {
			return result;
		}
	}

	if token.typ == "Variable" {
		let token_local = lexer.next_token();
		if token_local.value == "Erro"{eprintln!("Erro 525: Faltando tokens");exit(1);}
		match hm_variables.get_mut(&token.value){
			Some(tkn) => {
				let typ = &*tkn.1;
				match typ {
					"Bool" => {
						match hm_bools.get_mut(&token.value){
							Some(tkn_bool)=>{
								return Ret_variable{value: tkn_bool.clone().to_string(), typ: Typ::Bool};
							},
							None=>{
								eprintln!("Erro: Variavel '{}' não encontrada", token.value);
								exit(1);
							},
						}
					},
					"Number" => {
						match hm_numbers.get_mut(&token.value){
							Some(tkn_number)=>{
								return Ret_variable{value: tkn_number.clone().to_string(), typ: Typ::Number};
							},
							None=>{
								eprintln!("Erro: Variavel '{}' não encontrada", token.value);
								exit(1);
							},
						}
					},
					"String" => {
						match hm_strings.get_mut(&token.value){
							Some(tkn_string)=>{
								return Ret_variable{value: tkn_string.clone(), typ: Typ::String};
							},
							None=>{
								eprintln!("Erro: Variavel '{}' não encontrada", token.value);
								exit(1);
							},
						}
					},
					_ => {
						eprintln!("Erro: '{}' tipo não encontrado", typ);
						exit(1);
					},
				}
			},
			None => {
				eprintln!("Erro: Variavel não declarada '{}'", token.value);
				exit(1);
			}
		}
	} else if token.typ == "Number" {
		let token_next = lexer.next_token();
			if token_next.value == "Erro"{eprintln!("Erro 525: Faltando tokens");exit(1);}
		return Ret_variable{value: token.value, typ: Typ::Number};
	} else if token.typ == "String" {
		let token_next = lexer.next_token();
			if token_next.value == "Erro"{eprintln!("Erro 529: Faltando tokens");exit(1);}
		return Ret_variable{value: token.value, typ: Typ::String};
	} else if token.typ == "Reserved" {
		if token.value == "false" || token.value == "true"{
			let token_next = lexer.next_token();
			if token_next.value == "Erro"{eprintln!("Erro 534: Faltando tokens");exit(1);}
			return Ret_variable{value: token.value, typ: Typ::Bool};
		} else {
			eprintln!("Erro: Funcao {} nao pode ser atribuida", token.value);
			exit(1);
		}
	} else {
		eprintln!("Erro: Variavel não declarada '{}'", token.value);
		exit(1);
	}
}

// fn variable(){
// 	// [a-z] ([a-z] | [A-Z] | [0-9])+
// }

// fn typ(){
// 	// ("Number" | "String" | "Bool")
// }

// fn number(){
// 	// [0-9]+
// }

// fn string(){
// 	// """ ([a-z] | [A-Z] | [0-9])+ """
// }

// fn operator_low(){
// 	// ("+" | "-")
// }

// fn operator_high(){
// 	// ("*" | "/")
// }

// fn operator_atribuition(){
// 	// =
// }

// fn operator_comparation(){
// 	//( (("<" | ">" | "!") "="?) | "==") 
// }


fn main(){
		let mut f = File::open("./parser/src/teste3.txt").expect("file not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents)
		.expect("something went wrong reading the file");

	let mut lexer = Lexer::new(contents);
	lexer.generate_tokens();
	
	// for token in lexer.tokens {
 //       println!("{:?} {:?}\n", token.typ, token.value);
 //    }

	program(&mut lexer);
	
}
