pub struct Lexer {
	programa: String,
	pub tokens: Vec<Token>,
	pub cabecote_tokens: usize,
	tamanho: usize,
}

impl Lexer {
	pub fn new(programa: String) -> Lexer {
		Lexer {
			programa: programa,
			tokens: Vec::new(),
			cabecote_tokens: 0,
			tamanho: 0,
		}
	}

	pub fn next_token(&mut self) -> Token{
		self.cabecote_tokens = self.cabecote_tokens + 1;
		
		if self.cabecote_tokens == self.tamanho{
			let token = Token{typ: "Erro".to_string(), value: "Erro".to_string()};
			self.tokens.push(token);
			self.tamanho += 1; 
			
			
		}
		return self.tokens[self.cabecote_tokens].clone();
	}

	pub fn current_token(&mut self) -> Token {
		return self.tokens[self.cabecote_tokens].clone();
	}

	pub fn current_next_token(&mut self) -> Token {
		let next_index = self.cabecote_tokens + 1;
		if next_index == self.tamanho{
			let token = Token{typ: "Erro".to_string(), value: "Erro".to_string()};
			self.tokens.push(token);
			self.tamanho += 1; 
			
		}
		return self.tokens[next_index].clone();
	}

	pub fn current_previous_token(&mut self) -> Token {
		let mut previous_index = self.cabecote_tokens - 1;
		if previous_index < 0{
			let token = Token{typ: "Erro".to_string(), value: "Erro".to_string()};
			self.tokens.push(token);
			self.tamanho += 1; 
			previous_index = self.cabecote_tokens + 1;
		}
		
		return self.tokens[previous_index].clone();
	}

	pub fn define_cabecote(&mut self, new_value: usize){
		self.cabecote_tokens = new_value;
	}

	// fn verify line
	pub fn generate_tokens(&mut self) {
		let mut aux = String::new();
		// let mut list: Vec<char> = line_input.chars().collect();
		let mut list: Vec<char> = self.programa.chars().collect();
		list.push('\0');

		let mut list_tokens: Vec<Token> = Vec::new();
		
		let mut index = 0;
		// if list[index] == '\n' {list.remove(0);}
		
		while list[index] != '\0' {
			let state = 1;
			aux.push(list[index]);
			
			if list[index].is_alphabetic() {
				let (index_end, is_known) = self.mt_assignment(&list, index);
				if index_end == 0 {println!("Deu ruim1");}
				let value = list[index..index_end].into_iter().collect();
	
				let mut typ = 0;
				if is_known {
					typ = 4;
				} else if self.verify_reserved(&value) {
					typ = 6;
				} else if self.verify_type(&value) {
					typ = 9;
				} else if self.verify_create(&value) {
					typ = 10;
				} else {
					typ = 1;
				}
				
				list_tokens.push(self.create_token(typ, value));
				self.tamanho += 1;
				index = index_end;
			}
			else if list[index].is_numeric() {
				let (index_end, is_known) = self.mt_literal(1, &list, index);
				if index_end == 0 {println!("Deu ruim2");}
				let value = list[index..index_end].into_iter().collect();
				
				let mut typ = 0;
				if is_known {
					typ = 4;
				} else {
					typ = 2;
				}
				list_tokens.push(self.create_token(typ, value));
				self.tamanho += 1;
				index = index_end;
				
			}
			else if self.verify_operator(&list, index) {
				let (index_end, is_known) = self.mt_operator(state, &list, index, list[index]);
				if index_end == 0 {println!("Deu ruim3");}
				let value = list[index..index_end].into_iter().collect();
				
				let mut typ = 0;
				if is_known {
					typ = 4;
				} else {
					typ = 3;
				}
				
				list_tokens.push(self.create_token(typ, value));
				self.tamanho += 1;
				index = index_end;
			}
			else if self.verify_space(&list[index].to_string()){
				aux.clear();
				index += 1;
			}
			//String
			else if list[index] == '"'{
				index+=1;
				let (index_end, is_known) = self.mt_string(&list, index);
				if index_end == 0 {println!("Deu ruim4");}
				let value = list[index..index_end].into_iter().collect();
				
				let mut typ = 0;
				if is_known {
					typ = 4;
				} else {
					typ = 7;
				}
				
				list_tokens.push(self.create_token(typ, value));
				self.tamanho += 1;
				index = index_end+1;
			}
			// OperatorDefinition => 8
			else if list[index] == ':'{
				let typ = 8;
				let value = list[index].to_string();
				list_tokens.push(self.create_token(typ, value));
				self.tamanho += 1;
				index +=1;
			}
			//Block
			else if self.verify_block(&list[index].to_string()){
				let typ = 11;
				let value = list[index].to_string();
				list_tokens.push(self.create_token(typ, value));
				self.tamanho += 1;
				index +=1;
			}
			//Endline
			else if list[index] == ';' {
				let typ = 5;
				let value = list[index].to_string();
				list_tokens.push(self.create_token(typ, value));
				self.tamanho += 1;
				index +=1;
			}
			else {
				let index_end = self.mt_unk(&list, index);
				if index_end == 0 {println!("Deu ruimUNK");}
				let value = list[index..index_end].into_iter().collect();
				
				list_tokens.push(self.create_token(4, value));
				self.tamanho += 1;
				index = index_end;
			}
		}
	
		self.tokens = list_tokens;
	}

	//# MT Variable
	fn mt_assignment(&self, list: &Vec<char>, mut index: usize) -> (usize, bool) {
		
		if list[index].is_alphanumeric() {
			index += 1;
			return self.mt_assignment(list, index);
		}
		else if list[index] == ' ' || list[index] == ';' || self.verify_operator(&list, index) || list[index] == ':' || self.verify_block(&list[index].to_string()) || self.verify_space(&list[index].to_string()) || list[index] == '\0' {
			return (index, false);
		}
		else {
			return (index, true);
		}
		
	
	}
	//# MT String
	fn mt_string(&self, list: &Vec<char>, mut index: usize) -> (usize, bool){
		if list[index] != '"' {
			index += 1;
			return self.mt_string(list, index);
		}
		else {
			return (index, false);
		}
	}

	//# MT Literal
	fn mt_literal(&self, state: i32, list: &Vec<char>, mut index: usize) -> (usize, bool) {
	
		match state {
			1 => { //se for um numero
				if list[index].is_numeric(){
					index += 1;
					return self.mt_literal(1, list, index);
				} else if list[index] == '.'{
					index += 1;
					return self.mt_literal(2, list, index);
				} else if list[index] == 'e' {
					index += 1;
					return self.mt_literal(3, list, index);
				} else if list[index] == ' ' || list[index] == ';' || self.verify_operator(&list, index) || self.verify_block(&list[index].to_string()) || self.verify_space(&list[index].to_string()) {
					return self.mt_literal(7, list, index);
				} else {
					index += 1;
					return self.mt_literal(5, list, index);
				}
			},
			2 =>{ //se for um ponto
				if list[index].is_numeric(){
					index += 1;
					return self.mt_literal(6, list, index);
				} else {
					index += 1;
					return self.mt_literal(5, list, index);
				}
			}, 
			3 =>{ //se for um e
				if list[index] == '+' || list[index] == '-'{
					index += 1;
					return self.mt_literal(4, list, index);
				} else {
					index += 1;
					return self.mt_literal(5, list, index);
				}
			}, 
			4 =>{ //se for um operador
				if list[index].is_numeric(){
					index += 1;
					return self.mt_literal(4, list, index);
				} else if list[index] == ' ' || list[index] == ';' || self.verify_operator(&list, index) || self.verify_block(&list[index].to_string()) || self.verify_space(&list[index].to_string()) || list[index] == '\0' {
					return self.mt_literal(7, list, index);
				} else {
					index += 1;
					return self.mt_literal(5, list, index);
				}
			}, 
			5 => { // se for UNK
				return (self.mt_unk(list, index), true);
			}, 
			6 =>{ // se for um numero dps do ponto 
				if list[index].is_numeric(){
					index += 1;
					return self.mt_literal(6, list, index);
				} else if list[index] == 'e' {
					index += 1;
					return self.mt_literal(3, list, index);
				} else if list[index] == ' ' || list[index] == ';' || self.verify_operator(&list, index) || self.verify_block(&list[index].to_string()) || self.verify_space(&list[index].to_string()) || list[index] == '\0' {
					return self.mt_literal(7, list, index);
				} else {
					index += 1;
					return self.mt_literal(5, list, index);
				}
			}, 
			7 =>{return (index, false)}, //Aceitacao
			_ =>return (index, true),
		}
	
	}

	//# MT Operator
	fn mt_operator(&self, state: i32, list: &Vec<char>, mut index: usize, last_char: char) -> (usize, bool) {
	
		match state {
			1 => { //se for o primeiro operador
				if list[index] == '<' || list[index] == '>' || list[index] == '!' || list[index] == '=' || list[index] == '+' || list[index] == '-' || list[index] == '*' || list[index] == '/' {
					index += 1;
					return self.mt_operator(2, list, index, list[index-1]);
				} else if self.verify_block(&list[index].to_string()) || list[index] == '/' {
					index += 1;
					return self.mt_operator(4, list, index, list[index-1]);
				} else {
					index += 1;
					return self.mt_operator(5, list, index, list[index-1]);
				}			
			},
			2 => { // se for o segundo operador
				if list[index] == '=' {
					index += 1;
					return self.mt_operator(3, list, index, list[index-1]);
				} if list[index] == ' ' || list[index] == ';' || list[index].is_alphanumeric() || self.verify_space(&list[index].to_string()) || list[index] == '\0' {
					return (index, false);
				} else {
					index += 1;
					return self.mt_operator(5, list, index, list[index-1]);
				}
				
			},
			3 => { // estado final 2 operadores
				if list[index] == ' ' || list[index] == ';' || list[index].is_alphanumeric() {
					return (index, false);
				} else {
					return self.mt_operator(5, list, index, list[index-1]);
				}
			},
			4 => { // estado final 1 operadores
				if list[index] == ' ' || list[index] == ';' || self.verify_operator(&list, index) || self.verify_block(&list[index].to_string()) || self.verify_space(&list[index].to_string()) || list[index].is_alphanumeric() || list[index] == '\0'{
					return (index, false);
				} else {
					return self.mt_operator(5, list, index, list[index-1]);
				}
			}
			5 => return (self.mt_unk(list, index), true),
			_ => return (index,true),
		}
	
	}

	// MT UNK
	fn mt_unk(&self, list: &Vec<char>, mut index:usize) -> usize{
		if list[index] == ' ' || list[index] == ';' || list[index] == '\0' || self.verify_space(&list[index].to_string()) {
			return index;
		} else {
			index += 1;
			return self.mt_unk(list, index);
		}
	}

	// Create/Config Token
	fn create_token(&self, typ: i32, value:String) -> Token {
		let mut token = Token{typ: "typ".to_string(), value: value};
		
		token.typ = match typ {
			1 => "Variable",
			2 => "Number",
			3 => "Operator",
			4 => "UNK",
			5 => "EndLine",
			6 => "Reserved",
			7 => "String",
			8 => "OperatorDefinition",
			9 => "Type",
			10 => "Create",
			11 => "Block",
			_ => "404 type",
		}.to_string();
	
		return token;
	}
	// fn verify is operator
	fn verify_operator(&self, list: &Vec<char>, index: usize) -> bool {
		let list_operators = ['+', '-', '/', '*', '=', '<', '>', '!'];
	
		return list_operators.iter().find(|&&x| x == list[index]).is_some();
	}

	// fn verify isReserved
	fn verify_reserved(&self, value: &String ) -> bool{
		let list_reserved = [
			"main",
			"while",
			"if", "else",
			"print",
			"false", "true",
		];

		let string_reserved:Vec<String> = list_reserved.iter().map(|&x| x.to_string()).collect();
		
		return string_reserved.contains(value);
	}

	fn verify_type(&self, value: &String ) -> bool{
		let list_type = [
			"Bool",
			"Number",
			"String"
		];

		let string_type:Vec<String> = list_type.iter().map(|&x| x.to_string()).collect();
		
		return string_type.contains(value);
	}

	fn verify_create(&self, value: &String ) -> bool{
		let list_create = [
			"const",
			"let"
		];

		let string_create:Vec<String> = list_create.iter().map(|&x| x.to_string()).collect();
		
		return string_create.contains(value);
	}

	fn verify_block(&self, value: &String ) -> bool{
		let list_block = [
			"(", ")",
			"{", "}",
		];

		let string_block:Vec<String> = list_block.iter().map(|&x| x.to_string()).collect();
		
		return string_block.contains(value);
	}

	fn verify_space(&self, value: &String ) -> bool{
		let list_space = [
			' ', 
			'\t',
			'\n',
		];

		let string_space:Vec<String> = list_space.iter().map(|&x| x.to_string()).collect();
		
		return string_space.contains(value);
	}

	
}


// struct main
#[derive(Clone)]
pub struct Token {
	pub typ: String,
	pub value: String,
}



// // fn main
// fn main() {
// 	let mut f = File::open("./src/testes.txt").expect("file not found");
// 	let mut contents = String::new();
// 	f.read_to_string(&mut contents)
// 		.expect("something went wrong reading the file");

// 	let mut lexer = Lexer::new(contents);
// 	lexer.generate_tokens();
// 	println!("{:?}", lexer.current_token().typ);

// }



