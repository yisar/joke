use lexer;

#[derive(Debug, Clone)]
pub struct Parser {
    pub lexer: lexer::Lexer,
}

impl Parser {
    pub fn new(code: String) -> Parser {
        Parser {
            lexer: lexer::Lexer::new(code),
        }
    }
}

// impl Parser {
//     pub fn read_script(&mut self) -> Result<None,()>{
//         let mut items = vec![];

//         loop {
//             unimplemented!();
//         }
//     }
// }
