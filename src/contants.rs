use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Tokens {
    pub name: String,
    pub class: String,
    pub content: String,
}

pub fn asm_data() -> HashMap<&'static str, Vec<&'static str>> {
    let mut map = HashMap::new();
    map.insert(
        "operators",
        vec![
            "+", "-", "/", "*", "(", ")", "[", "]", "\"", "'", ",", ".", "%", "=", "==", "<", ">",
            "!", "!=", "<=", ">=", "+=", "-=", "*=", "/=", "&", "&&", "|", "||", "^", "~",
        ],
    );
    map.insert(
        "arithmetics",
        vec![
            "ADD", "SUB", "INC", "DEC", "MUL", "IMUL", "DIV", "IDIV", "AND", "OR", "XOR", "NOT",
            "SHL", "SHR",
        ],
    );
    map.insert(
        "criticals",
        vec!["RET", "PROC", "ENDP", "END", "INCLUDE", "SECTION"],
    );
    map.insert(
        "instructions",
        vec![
            "MOV", "MOVS", "MOVSX", "MOVZX", "CMP", "PUSH", "POP", "PUSHAD", "POPAD", "LEA", "NOP",
            "HLT", "INT", "LEAVE", "CLC", "STC", "CLD", "STD", "CLI", "STI", "CMPXCHG", "XCHG",
            "BSWAP", "NOP", "PUSHF", "POPF", "REP", "REPE", "REPZ", "REPNE", "REPNZ", "CMC",
            "CWDE", "CDQ", "WAIT", "CBW", "CWD", "INTO", "IRET", "OFFSET", "PTR", "FLD", "FSTP",
            "SYSCALL", "USES", "COMMENT", "EQU", "GLOBAL",
        ],
    );
    map.insert(
        "datatypes",
        vec![
            "BYTE", "WORD", "DWORD", "QWORD", "DB", "DW", "DD", "DQ", "REAL", "RESB", "RESW",
            "RESD", "RESQ",
        ],
    );
    map.insert(
        "jumps",
        vec![
            "JMP", "JE", "JNE", "JG", "JGE", "JL", "JLE", "JZ", "JNZ", "JS", "JNS", "JC", "JNC",
            "JB", "JA", "CALL", "INVOKE",
        ],
    );
    map.insert(
        "registers",
        vec![
            "AL", "BL", "CL", "DL", "AH", "BH", "CH", "DH", "AX", "BX", "CX", "DX", "EAX", "EBX",
            "ECX", "EDX", "RAX", "RBX", "RCX", "RDX", "DI", "SI", "EDI", "ESI", "EBP", "ESP",
            "RBP", "RSP", "RDI", "RSI", "R8", "R9", "R10", "R11", "R12", "R13", "R14", "R15",
            "R8B", "R8W", "R8D", "R9B", "R9W", "R9D", "R10B", "R10W", "R10D", "R11B", "R11W",
            "R11D", "R12B", "R12W", "R12D", "R13B", "R13W", "R13D", "R14B", "R14W", "R14D", "R15B",
            "R15W", "R15D",
        ],
    );
    map
}
