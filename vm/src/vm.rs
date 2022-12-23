use std::ops::{Deref, DerefMut};
use crate::const_table::ConstTable;
use crate::opcode::OpCode;
use crate::stack::VmStack;
use crate::types::*;
use crate::errors::*;

pub struct VM {
    stack:VmStack,
    pc:usize,
    op_codes:Vec<OpCode>,
    const_table:ConstTable,
}

impl VM{
    pub fn new(op_codes:Vec<OpCode>,const_table:Vec<Value>)->Self{
        Self {
            stack: Default::default(),
            pc: 0,
            op_codes,
            const_table:ConstTable::new(const_table)
        }
    }

    pub fn register(&self, reg:u8) -> &RegType{
        self.stack.register(reg)
    }
    pub fn register_mut(&mut self, reg:u8) -> &mut RegType{
        self.stack.register_mut(reg)
    }
    pub fn execute_code(&mut self,op:OpCode)->Result<()>{
        match op {
            OpCode::Or(a, b, c) => {
                *self.register_mut(a) = op_or(self.register(b), self.register(c))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::And(a, b, c) => {
                *self.register_mut(a) = op_and(self.register(b),self.register(c))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::BitOr(a, b, c) => {
                *self.register_mut(a) = op_bit_or(self.register(b),self.register(c))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::BitXor(a, b, c) => {
                *self.register_mut(a) = op_bit_xor(self.register(b),self.register(c))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::BitAnd(a, b, c) => {
                *self.register_mut(a) = op_bit_and(self.register(b),self.register(c))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::NE(a, b, c) => {
                *self.register_mut(a) = op_ne(self.register(b),self.register(c))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::EQ(a, b, c) => {
                *self.register_mut(a) = op_eq(self.register(b),self.register(c))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::RefEQ(a, b, c) => { unimplemented!() }
            OpCode::RefNE(_, _, _) => {unimplemented!()}
            OpCode::LT(a, b, c) => {
                *self.register_mut(a) = op_lt(self.register(b),self.register(c))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::GT(a, b, c) => {
                *self.register_mut(a) = op_gt(self.register(b),self.register(c))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::LE(a, b, c) => {
                *self.register_mut(a) = op_le(self.register(b),self.register(c))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::GE(a, b, c) => {
                *self.register_mut(a) = op_le(self.register(b),self.register(c))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::LMov(a , b, c) => {
                *self.register_mut(a) = op_l_mov(self.register(b),self.register(c))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::RMov(a, b, c) => {
                *self.register_mut(a) = op_r_mov(self.register(b),self.register(c))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::Add(a, b, c) => {
                *self.register_mut(a) = op_and(self.register(b),self.register(c))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::Sub(a, b, c) => {
                *self.register_mut(a) = op_and(self.register(b),self.register(c))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::Mul(a, b, c) => {
                *self.register_mut(a) = op_mul(self.register(b),self.register(c))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::Div(a, b, c) => {
                *self.register_mut(a) = op_div(self.register(b),self.register(c))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::Mod(a, b, c) => {
                *self.register_mut(a) = op_mod(self.register(b),self.register(c))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::Fact(a, b, c) => {
                *self.register_mut(a) = op_fact(self.register(b),self.register(c))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::BitNot(a, b) => {
                *self.register_mut(a) = op_bit_not(self.register(b))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::Not(a, b) => {
                *self.register_mut(a) = op_not(self.register(b))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::Neg(a, b) => {
                *self.register_mut(a) = op_neg(self.register(b))?.load_variable(true);
                self.pc+=1;
            }
            OpCode::Pos(a, b) => {
                *self.register_mut(a) = op_pos(self.register(b))?.load_variable(true);
                self.pc+=1;
            }
            // OpCode::ArrayVisit(_, _, _) => {unimplemented!()}
            // OpCode::MemberGet(_, _, _) => {unimplemented!()}
            // OpCode::MemberSet(_, _, _) => {unimplemented!()}
            OpCode::RefAssign(a, b) => {
                unimplemented!()
            }
            OpCode::ValAssign(a, b) => {
                //todo op_assign(self.register_mut(a),self.register(b))?;
                self.pc+=1;
            }
            OpCode::JmpPrev(a,b,c)  => {
                let pos = OpCode::JmpPrev(a,b,c).get_u24();
                self.pc -= pos as usize;
            }
            OpCode::JmpPost(a,b,c)  => {
                let pos = OpCode::JmpPost(a,b,c).get_u24();
                self.pc += pos as usize;
            }
            OpCode::Chk(a)  => {

            }
            OpCode::Call(_) => {unimplemented!()}
            OpCode::CallConst0(_) => {unimplemented!()}
            OpCode::Ret => {unimplemented!()}

            OpCode::LoadAsMutRef(a, addr) => {
                *self.register_mut(a) = self.const_table.load_variable(addr,true);
                self.pc += 1;
            }
            OpCode::LoadAsConstRef( a, addr) => {
                *self.register_mut(a) = self.const_table.load_variable(addr,false);
                self.pc += 1;
            }
        }
        Ok(())
    }

    pub fn run(&mut self){
        loop {
            self.execute_code(self.op_codes[self.pc]).unwrap();
            println!("{:?}",self.stack)
        }
    }

}