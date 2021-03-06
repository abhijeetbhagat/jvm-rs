use instruction::instruction::ExecuteResult;
use rtda::frame::Frame;
use util::code_reader::CodeReader;

#[allow(non_snake_case)]
fn _icmpPop(frame: Frame) -> (i32, i32, Frame) {
    let Frame {
        operand_stack,
        local_vars,
    } = frame;

    let (val2, operand_stack) = operand_stack.pop_int();
    let (val1, operand_stack) = operand_stack.pop_int();

    let frame = Frame {
        operand_stack,
        local_vars,
    };
    (val1, val2, frame)
}

#[allow(non_snake_case)]
pub fn IF_ICMPGT(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("IF_ICMPGT");
    let (offset, code_reader) = code_reader.read_i16();

    let (val1, val2, frame) = _icmpPop(frame);
    let offset = if val1 > val2 { offset as isize } else { 0 };

    let execute_result = ExecuteResult { frame, offset };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn IF_ICMPGE(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("IF_ICMPGE");
    let (offset, code_reader) = code_reader.read_i16();

    let (val1, val2, frame) = _icmpPop(frame);
    let offset = if val1 >= val2 { offset as isize } else { 0 };

    let execute_result = ExecuteResult { frame, offset };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn IF_ICMPEQ(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("IF_ICMPEQ");
    let (offset, code_reader) = code_reader.read_i16();

    let (val1, val2, frame) = _icmpPop(frame);
    let offset = if val1 == val2 { offset as isize } else { 0 };

    let execute_result = ExecuteResult { frame, offset };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn IF_ICMPNE(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("IF_ICMPNE");
    let (offset, code_reader) = code_reader.read_i16();

    let (val1, val2, frame) = _icmpPop(frame);
    let offset = if val1 != val2 { offset as isize } else { 0 };

    let execute_result = ExecuteResult { frame, offset };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn IF_ICMPLT(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("IF_ICMPLT");
    let (offset, code_reader) = code_reader.read_i16();

    let (val1, val2, frame) = _icmpPop(frame);
    let offset = if val1 < val2 { offset as isize } else { 0 };

    let execute_result = ExecuteResult { frame, offset };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn IF_ICMPLE(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("IF_ICMPLE");
    let (offset, code_reader) = code_reader.read_i16();

    let (val1, val2, frame) = _icmpPop(frame);
    let offset = if val1 <= val2 { offset as isize } else { 0 };

    let execute_result = ExecuteResult { frame, offset };
    (execute_result, code_reader)
}

#[cfg(test)]
mod tests {
    use instruction::comparison::ifcond::*;
    use instruction::instruction::ExecuteResult;
    use rtda::frame::Frame;
    use util::code_reader::CodeReader;

    #[test]
    #[allow(non_snake_case)]
    fn test_ICMPGT_success() {
        let frame = Frame::new(2, 2);
        let Frame {
            operand_stack,
            local_vars,
        } = frame;

        let operand_stack = operand_stack.push_int(1);
        let operand_stack = operand_stack.push_int(0);
        let frame = Frame {
            operand_stack,
            local_vars,
        };
        let (ExecuteResult { frame: _, offset }, _) = IFEQ(CodeReader::new(&vec![1, 1]), frame);
        assert_eq!(offset, 257);
    }

    #[allow(non_snake_case)]
    fn test_ICMPGT_fail() {
        let frame = Frame::new(2, 2);
        let Frame {
            operand_stack,
            local_vars,
        } = frame;

        let operand_stack = operand_stack.push_int(1);
        let operand_stack = operand_stack.push_int(2);
        let frame = Frame {
            operand_stack,
            local_vars,
        };
        let (ExecuteResult { frame: _, offset }, _) = IFEQ(CodeReader::new(&vec![1, 1]), frame);
        assert_eq!(offset, 0);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_ICMPGE_success() {
        let frame = Frame::new(2, 2);
        let Frame {
            operand_stack,
            local_vars,
        } = frame;

        let operand_stack = operand_stack.push_int(1);
        let operand_stack = operand_stack.push_int(1);
        let frame = Frame {
            operand_stack,
            local_vars,
        };
        let (ExecuteResult { frame: _, offset }, _) = IFNE(CodeReader::new(&vec![1, 1]), frame);
        assert_eq!(offset, 257);
    }

    #[allow(non_snake_case)]
    fn test_ICMPGE_fail() {
        let frame = Frame::new(2, 2);
        let Frame {
            operand_stack,
            local_vars,
        } = frame;

        let operand_stack = operand_stack.push_int(0);
        let operand_stack = operand_stack.push_int(2);
        let frame = Frame {
            operand_stack,
            local_vars,
        };
        let (ExecuteResult { frame: _, offset }, _) = IFNE(CodeReader::new(&vec![1, 1]), frame);
        assert_eq!(offset, 0);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_ICMPEQ_success() {
        let frame = Frame::new(2, 2);
        let Frame {
            operand_stack,
            local_vars,
        } = frame;

        let operand_stack = operand_stack.push_int(-1);
        let operand_stack = operand_stack.push_int(-1);
        let frame = Frame {
            operand_stack,
            local_vars,
        };
        let (ExecuteResult { frame: _, offset }, _) = IFLT(CodeReader::new(&vec![1, 1]), frame);
        assert_eq!(offset, 257);
    }

    #[allow(non_snake_case)]
    fn test_ICMPEQ_fail() {
        let frame = Frame::new(2, 2);
        let Frame {
            operand_stack,
            local_vars,
        } = frame;

        let operand_stack = operand_stack.push_int(0);
        let operand_stack = operand_stack.push_int(1);
        let frame = Frame {
            operand_stack,
            local_vars,
        };
        let (ExecuteResult { frame: _, offset }, _) = IFLT(CodeReader::new(&vec![1, 1]), frame);
        assert_eq!(offset, 0);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_ICMPNE_success() {
        let frame = Frame::new(2, 2);
        let Frame {
            operand_stack,
            local_vars,
        } = frame;

        let operand_stack = operand_stack.push_int(0);
        let operand_stack = operand_stack.push_int(1);
        let frame = Frame {
            operand_stack,
            local_vars,
        };
        let (ExecuteResult { frame: _, offset }, _) = IFGE(CodeReader::new(&vec![1, 1]), frame);
        assert_eq!(offset, 257);
    }

    #[allow(non_snake_case)]
    fn test_ICMPNE_fail() {
        let frame = Frame::new(2, 2);
        let Frame {
            operand_stack,
            local_vars,
        } = frame;

        let operand_stack = operand_stack.push_int(-1);
        let operand_stack = operand_stack.push_int(-1);
        let frame = Frame {
            operand_stack,
            local_vars,
        };
        let (ExecuteResult { frame: _, offset }, _) = IFGE(CodeReader::new(&vec![1, 1]), frame);
        assert_eq!(offset, 0);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_ICMPLT_success() {
        let frame = Frame::new(2, 2);
        let Frame {
            operand_stack,
            local_vars,
        } = frame;

        let operand_stack = operand_stack.push_int(1);
        let operand_stack = operand_stack.push_int(2);
        let frame = Frame {
            operand_stack,
            local_vars,
        };
        let (ExecuteResult { frame: _, offset }, _) = IFGT(CodeReader::new(&vec![1, 1]), frame);
        assert_eq!(offset, 257);
    }

    #[allow(non_snake_case)]
    fn test_ICMPLT_fail() {
        let frame = Frame::new(2, 2);
        let Frame {
            operand_stack,
            local_vars,
        } = frame;

        let operand_stack = operand_stack.push_int(0);
        let operand_stack = operand_stack.push_int(0);
        let frame = Frame {
            operand_stack,
            local_vars,
        };
        let (ExecuteResult { frame: _, offset }, _) = IFGT(CodeReader::new(&vec![1, 1]), frame);
        assert_eq!(offset, 0);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_ICMPLE_success() {
        let frame = Frame::new(2, 2);
        let Frame {
            operand_stack,
            local_vars,
        } = frame;

        let operand_stack = operand_stack.push_int(0);
        let operand_stack = operand_stack.push_int(0);
        let frame = Frame {
            operand_stack,
            local_vars,
        };
        let (ExecuteResult { frame: _, offset }, _) = IFLE(CodeReader::new(&vec![1, 1]), frame);
        assert_eq!(offset, 257);
    }

    #[allow(non_snake_case)]
    fn test_ICMPLE_fail() {
        let frame = Frame::new(2, 2);
        let Frame {
            operand_stack,
            local_vars,
        } = frame;

        let operand_stack = operand_stack.push_int(1);
        let operand_stack = operand_stack.push_int(2);
        let frame = Frame {
            operand_stack,
            local_vars,
        };
        let (ExecuteResult { frame: _, offset }, _) = IFLE(CodeReader::new(&vec![1, 1]), frame);
        assert_eq!(offset, 0);
    }

}
