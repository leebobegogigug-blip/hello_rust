// ==============================================================================
// Rust 첫걸음: 핵심 문법 체험 (Struct, Enum, Pattern Matching, Result, Test)
// ==============================================================================

#[derive(Debug, PartialEq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub struct Calculation {
    pub a: f64,
    pub b: f64,
    pub op: Operation,
}

impl Calculation {
    pub fn new(a: f64, b: f64, op: Operation) -> Self {
        Self { a, b, op }
    }

    pub fn execute(&self) -> Result<f64, &'static str> {
        match self.op {
            Operation::Add => Ok(self.a + self.b),
            Operation::Subtract => Ok(self.a - self.b),
            Operation::Multiply => Ok(self.a * self.b),
            Operation::Divide => {
                if self.b == 0.0 {
                    Err("0으로 나눌 수 없습니다 (Division by zero)")
                } else {
                    Ok(self.a / self.b)
                }
            }
        }
    }
}

fn main() {
    println!("🦀 Rust 개발 환경 세팅 완료!\n");

    let calculations = vec![
        Calculation::new(10.0, 5.0, Operation::Add),
        Calculation::new(20.0, 4.0, Operation::Divide),
        Calculation::new(7.0, 0.0, Operation::Divide), // 에러 핸들링 케이스
    ];

    for calc in calculations {
        print!("{:?}({}, {}) => ", calc.op, calc.a, calc.b);
        match calc.execute() {
            Ok(result) => println!("성공: {:.2}", result),
            Err(err) => println!("오류: {}", err),
        }
    }

    println!("\n💡 다음 명령어들을 시도해보세요:");
    println!("  cargo run   - 프로그램 빌드 및 실행");
    println!("  cargo test  - 유닛 테스트 실행");
    println!("  cargo check - 초고속 컴파일 문법 검사");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let calc = Calculation::new(2.5, 3.5, Operation::Add);
        assert_eq!(calc.execute().unwrap(), 6.0);
    }

    #[test]
    fn test_divide_by_zero() {
        let calc = Calculation::new(10.0, 0.0, Operation::Divide);
        assert!(calc.execute().is_err());
    }
}
