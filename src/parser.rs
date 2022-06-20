use std::{fs,process::exit};
use crate::token::TokenType;
use aho_corasick::AhoCorasick;
//enum Registers64{
//    // temp/return
//    rax{value:Value},
//    // calee saved
//    rbx{value:Value},
//    /// rcx rdx rsi rdi r8 r9 are all used to pass function arguments so I will just comment the
//    /// number argument they put in
//    rcx{value:Value}, // 4
//    rdx{value:Value}, // 3
//    rsp{value:Value}, // stack pointer
//    rbp{value:Value}, // callee saved ; base pointer
//    rsi{value:Value}, // 2
//    rdi{value:Value}, // 1
//    r8{value:Value}, // 5
//    r9{value:Value}, // 6
//    r10{value:Value}, // temp
//    r11{value:Value},// temp
//    // r12-15 are all callee registers
//    r12{value:Value},
//    r13{value:Value},
//    r14{value:Value},
//    r15{value:Value},
//}
//enum fRegister {
//    xmm0{value:f64},
//    xmm1{value:f64},
//    xmm2{value:f64},
//    xmm3{value:f64},
//    xmm4{value:f64},
//    xmm5{value:f64},
//    xmm6{value:f64},
//    xmm7{value:f64},
//}
//enum Value {
//    string(String),
//    int(i64),
//    uint(u64),
//    fregister(fRegister),
//    register(Box<Registers64>),
//}
//enum AsmInstructions {
//  //movl{value:i32,register:Registers32},// this is a 64 bit lang I dont want to work on 32 bit rn
//    movq{value:Value,register:Registers64},// 64 bit mov
//    ascii{value:String},
//    call{label:String},
//    addq{a:Value,b:Value},
//    subq{a:Value,b:Value},
//    imulq{a:Value,b:Value},
//    idivq{a:Value,b:Value},
//    xor{a:Value,b:Value},
//}
struct AstNode {
    value:String,
    children:Vec<AstNode>,
}
impl AstNode {
    fn new(v:String,children:Vec<AstNode>) -> Self {
        Self {
            value:v,
            children:children,
        }
    }
}
pub fn create_ast () {
    let file:String = fs::read_to_string("lexed.frl").expect("cant find lexed file");
    let types = get_types(file);
    if !types.contains(&31) {
        println!("\nNo Entrypoint found consider adding \"!!\" after your main function declaration");
        exit(1);
    }
    let mut i = 0;
    let mut ast:Vec<AstNode>;
    match types[0] {
        0..=5 => {
            println!("FATAL ERROR: during parsing: file cannot begin with !,|,^,&,!&,or an Identifier");
            exit(1);
        },
        17..=30 => {
            println!("FATAL ERROR: druing parsing: file cannot begin with !,=,<,>,(,),+,-,%,/,*,;,:,or \",\"");
            exit(1);
        }
        _ => (),
    }
    let typesiter = types.iter();
println!("\n{:?}",types);
}
fn get_types (s:String) -> Vec<usize> {
    // substrings to find 
    let dictionary = &["Not","Or","XOr","Nand","And","Identifier","STRING","CHAR","IntNumber","FloatNumber","Res_Bool","Res_Int","Res_Uint","Res_Char","Res_Float","Res_String","Res_Function","Bang","Equal","Larrow","LParen","Rarrow","Rparen","Add","Subtract","Modulo","Divide","Multiply","SemiColon","Colon","Comma","Main"];

let mat = AhoCorasick::new(dictionary);

// matches array 
let mut matches:Vec<usize> = vec![];
for m in mat.find_iter(&s) {
    matches.push(m.pattern());
    }
    return matches;
}
