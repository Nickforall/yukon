use easter;
use easter::stmt::StmtListItem;
use easter::stmt::Stmt;
use easter::expr::Expr;
use easter::punc::BinopTag;

#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    PUSHNUM(f64),
    PUSHSTRLIT(String),
    ADD,
    SUB,
    MLP,
    DIV,
    PUSHTRUE,
    PUSHFALSE,
    EQ,
    SEQ,
    NEQ,
    SNEQ,
}

#[derive(Debug, PartialEq)]
pub struct Image {
    pub script: Block,
    pub blocks: Vec<Block>,
}

impl Image {
    pub fn new() -> Image {
        Image {
            script: Block { instructions: Vec::new() },
            blocks: Vec::new(),
        }
    }

    pub fn push_instruction(&mut self, instr: Instruction) {
        self.script.instructions.push(instr);
    }

    pub fn push_number(&mut self, num: f64) {
        self.script.instructions.push(Instruction::PUSHNUM(num));
    }

    pub fn push_string(&mut self, s: String) {
        self.script.instructions.push(Instruction::PUSHSTRLIT(s));
    }
}

#[derive(Debug, PartialEq)]
pub struct Block {
    pub instructions: Vec<Instruction>
}

pub fn compile_to_image(body: Vec<easter::stmt::StmtListItem>) -> Image {
    let mut image = Image::new();

    for stmt_item in body {
        // println!("{:#?}", stmtItem);

        match stmt_item {
            StmtListItem::Decl(decl) => compile_decl(&mut image, decl),
            StmtListItem::Stmt(stmt) => compile_stmt(&mut image, stmt),
        }
    }

    image
}

fn compile_decl(image: &mut Image, decl: easter::decl::Decl) {

}

fn compile_stmt(image: &mut Image, stmt: Stmt) {
    match stmt {
        Stmt::Expr(_, expr, _) => compile_expression(image, expr),
        _ => panic!("Unsupported statement"),
    }
}

fn compile_expression(image: &mut Image, expr: Expr) {
    match expr {
        Expr::Binop(_, op, left, right) => {
            compile_expression(image, *right);
            compile_expression(image, *left);
            compile_bin_op(image, op);
        },
        Expr::Number(_, number) => image.push_number(number.value),
        Expr::String(_, string_literal) => image.push_string(string_literal.value),
        Expr::True(_) => image.push_instruction(Instruction::PUSHTRUE),
        Expr::False(_) => image.push_instruction(Instruction::PUSHFALSE),
        _ => panic!("Unsupported expression"),
    }
}

fn compile_bin_op(image: &mut Image, binop: easter::punc::Binop) {
    match binop.tag {
        BinopTag::Plus => image.push_instruction(Instruction::ADD),
        BinopTag::Minus => image.push_instruction(Instruction::SUB),
        BinopTag::Times => image.push_instruction(Instruction::MLP),
        BinopTag::Div => image.push_instruction(Instruction::DIV),
        BinopTag::StrictEq => image.push_instruction(Instruction::SEQ),
        BinopTag::Eq => image.push_instruction(Instruction::EQ),
        BinopTag::StrictNEq => image.push_instruction(Instruction::SNEQ),
        BinopTag::NEq => image.push_instruction(Instruction::NEQ),

        _ => panic!("Unsupported statement"),
    }
}
