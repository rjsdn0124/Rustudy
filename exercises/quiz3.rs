// quiz3.rs
// This quiz tests:
// - Generics
// - Traits
// An imaginary magical school has a new report card generation system written in Rust!
// Currently the system only supports creating report cards where the student's grade
// is represented numerically (e.g. 1.0 -> 5.5).
// However, the school also issues alphabetical grades (A+ -> F-) and needs
// to be able to print both types of report card!

// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to "A+"
// to show that your changes allow alphabetical grades.

// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

// std::fmt::Display를 안하면 format 함수 내에서 제너릭을 사용하지 못함!
// 확실한거는 제너릭의 T는 따로 선언을 안해주면 변수 값을 초기화한데서 자동으로 가져오는 거 같아..
// 그리고 impl하면서 제너릭 전달은 객체 전체 스코프에서 돌아가서 객체<T>를 해줘야하는듯!
// impl에서는 해당 선언에서 쓸 모든 제너릭을 다 받아서 저장하는듯! 뿜빠이빠이야!

pub struct ReportCard<T> {
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

impl<T: std::fmt::Display> ReportCard<T> {
    pub fn print(& self) -> String {
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade)
    }
}
trait UpdateGrade<T>{
    fn update_grade(self, grade:T) -> ReportCard<T>;
}
impl<T1,T2> UpdateGrade<T2> for ReportCard<T1>{
    fn update_grade(self, grade:T2) -> ReportCard<T2> {
        ReportCard{
            grade: grade,
            student_name:self.student_name,
            student_age:self.student_age,
        }
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
            grade: 2.1,
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        let report_card = report_card.update_grade("A+".to_string());
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
