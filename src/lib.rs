pub mod builder;
pub mod bytecode;
pub mod runtime;

#[cfg(test)]
mod tests {
    use crate::{
        builder::BytecodeBuilder,
        bytecode::Opcode,
        runtime::{Value, vm::Vm},
    };

    fn assert_vm_output(builder: &mut BytecodeBuilder, expected: Value) {
        let bytecode = builder.finish_and_reset();
        let mut vm = Vm::new(bytecode);

        vm.run().unwrap();

        assert_eq!(vm.peek(), Some(&expected));
    }

    #[test]
    fn all() {
        let mut builder = BytecodeBuilder::new();

        print_int(&mut builder);
        debug_print_int(&mut builder);

        add_ints(&mut builder);
        sub_ints(&mut builder);
        mul_ints(&mut builder);
        div_ints(&mut builder);
        rem_ints(&mut builder);
    }

    fn print_int(builder: &mut BytecodeBuilder) {
        builder
            .ldc(Value::Int(42))
            .op(Opcode::Print)
            .op(Opcode::Halt);

        let bytecode = builder.finish_and_reset();
        let mut vm = Vm::new(bytecode);

        vm.run().unwrap();
    }

    fn debug_print_int(builder: &mut BytecodeBuilder) {
        builder
            .ldc(Value::Int(42))
            .op(Opcode::DebugPrint)
            .op(Opcode::Halt);

        let bytecode = builder.finish_and_reset();
        let mut vm = Vm::new(bytecode);

        vm.run().unwrap();
    }

    fn add_ints(builder: &mut BytecodeBuilder) {
        builder
            .ldc(Value::Int(10))
            .ldc(Value::Int(20))
            .op(Opcode::Add)
            .op(Opcode::Halt);

        assert_vm_output(builder, Value::Int(30));
    }

    fn sub_ints(builder: &mut BytecodeBuilder) {
        builder
            .ldc(Value::Int(20))
            .ldc(Value::Int(10))
            .op(Opcode::Sub)
            .op(Opcode::Halt);

        assert_vm_output(builder, Value::Int(10));
    }

    fn mul_ints(builder: &mut BytecodeBuilder) {
        builder
            .ldc(Value::Int(10))
            .ldc(Value::Int(5))
            .op(Opcode::Mul)
            .op(Opcode::Halt);

        assert_vm_output(builder, Value::Int(50));
    }

    fn div_ints(builder: &mut BytecodeBuilder) {
        builder
            .ldc(Value::Int(20))
            .ldc(Value::Int(4))
            .op(Opcode::Div)
            .op(Opcode::Halt);

        assert_vm_output(builder, Value::Int(5));
    }

    fn rem_ints(builder: &mut BytecodeBuilder) {
        builder
            .ldc(Value::Int(17))
            .ldc(Value::Int(5))
            .op(Opcode::Rem)
            .op(Opcode::Halt);

        assert_vm_output(builder, Value::Int(2));
    }
}