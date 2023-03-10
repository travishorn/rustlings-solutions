// An imaginary magical school has a new report card generation system written in Rust! Currently
// the system only supports creating report cards where the student's grade is represented
// numerically (e.g. 1.0 -> 5.5). However, the school also issues alphabetical grades (A+ -> F-) and
// needs to be able to print both types of report card!

// The struct ReportCard and the impl block support alphabetical report cards. The Grade in the
// second test is "A+" to show that it allows alphabetical grades.

// This `ReportCard` struct supports a generic type `T`
pub struct ReportCard<T> {
    // `grade` uses the generic type `T` because it may be given an `f32` or a `String`
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

// To use the generic type `T`, both the `impl` and the name must be followed by `<T>`. We also need
// to define how to display the generic type, otherwise `&self.grade` cannot be formatted using
// `format!`. We do this with `<T: std::fmt::Display>`.
impl<T: std::fmt::Display> ReportCard<T> {
    pub fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade
        )
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
            grade: "A+".to_string(),
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
