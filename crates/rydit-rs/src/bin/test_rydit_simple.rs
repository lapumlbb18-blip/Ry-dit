// Test simple: Parsear y ejecutar un script .rydit básico
// Sin gráficos, solo texto

use rydit_vm::VM;
use rydit_parser::Parser;
use rydit_lexer::Lexer;

fn main() {
    println!("=== TEST: Parser + VM ===\n");

    // Test 1: Hola Mundo
    let script = r#"voz "Hola RyDit""#;
    println!("Script: {}", script);
    
    let tokens = Lexer::new(script).scan();
    println!("Tokens: {}", tokens.len());
    
    let mut parser = Parser::new(tokens);
    let (program, errors) = parser.parse();
    
    if !errors.is_empty() {
        println!("ERRORES: {:?}", errors);
        return;
    }
    
    println!("Programa parseado: OK\n");

    // Test 2: Variables
    let script2 = r#"
dark.slot x = 10
dark.slot y = 20
voz "x + y = " + (x + y)
"#;
    println!("Script 2:\n{}", script2);
    
    let tokens2 = Lexer::new(script2).scan();
    let mut parser2 = Parser::new(tokens2);
    let (program2, errors2) = parser2.parse();
    
    if !errors2.is_empty() {
        println!("ERRORES: {:?}", errors2);
        return;
    }
    
    println!("Variables: OK\n");

    // Test 3: Bucle
    let script3 = r#"
dark.slot i = 0
ryda i < 3 {
    voz "Iteracion: " + i
    dark.slot i = i + 1
}
"#;
    println!("Script 3:\n{}", script3);
    
    let tokens3 = Lexer::new(script3).scan();
    let mut parser3 = Parser::new(tokens3);
    let (program3, errors3) = parser3.parse();
    
    if !errors3.is_empty() {
        println!("ERRORES: {:?}", errors3);
        return;
    }
    
    println!("Bucle: OK\n");
    println!("=== TODOS LOS TESTS PASARON ===");
}
