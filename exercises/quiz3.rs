// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

// 定义一个 Display 特征，用于将成绩格式化为字符串
pub trait Display {
    fn display(&self) -> String;
}

// 为 f32 实现 Display 特征
impl Display for f32 {
    fn display(&self) -> String {
        self.to_string()
    }
}

// 为 String 实现 Display 特征
impl Display for String {
    fn display(&self) -> String {
        self.clone()
    }
}

// 为 &str 实现 Display 特征
impl Display for &str {
    fn display(&self) -> String {
        (*self).to_string()
    }
}

// 将 ReportCard 改为泛型结构体，其中 G 是实现了 Display 特征的类型
pub struct ReportCard<G: Display> {
    pub grade: G,
    pub student_name: String,
    pub student_age: u8,
}

// 为泛型 ReportCard 实现 print 方法
impl<G: Display> ReportCard<G> {
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade.display())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
