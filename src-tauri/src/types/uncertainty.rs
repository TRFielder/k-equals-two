use std::f64;

// Assumed probability distribution
pub enum Distribution {
    Rectangular,
    Triangular,
    //  Holds a custom divisor for normal distribution (likely 1, 2 or 3)
    Normal(f64),
}

impl Distribution {
    // Provide a divisor for each distribution
    fn divisor(&self) -> f64 {
        match self {
            Distribution::Rectangular => f64::sqrt(3.0f64),
            Distribution::Triangular => f64::sqrt(6.0f64),
            Distribution::Normal(k_value) => {
                if *k_value <= 0.0 {
                    panic!("k value for normal distribution must be greater than 0")
                }
                *k_value // Use the provided k value as the divisor
            }
        }
    }
}

pub struct UncertaintyTerm {
    pub name: String,
    pub description: String,
    pub value: f64,
    pub distribution: Distribution,
}

impl UncertaintyTerm {
    fn divisor(&self) -> f64 {
        self.distribution.divisor()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangular_distribution_divisor() {
        let distribution = Distribution::Rectangular;
        let expected_divisor = f64::sqrt(3.0f64);
        assert_eq!(distribution.divisor(), expected_divisor)
    }

    #[test]
    fn test_triangular_distribution_divisor() {
        let distribution = Distribution::Triangular;
        let expected_divisor = f64::sqrt(6.0f64);
        assert_eq!(distribution.divisor(), expected_divisor);
    }

    #[test]
    fn test_normal_distribution_divisor_with_multiple_kvalues() {
        // Define a list of k values to test
        let k_values = vec![1.0, 1.5, 2.0, 2.5, 3.0];

        for &k in &k_values {
            let distribution = Distribution::Normal(k);
            assert_eq!(distribution.divisor(), k, "Failed for k value: {}", k);
        }
    }

    #[test]
    #[should_panic]
    fn test_invalid_k_value() {
        let k: f64 = 0.0;

        let distribution = Distribution::Normal(k);
        distribution.divisor();
    }

    #[test]
    fn test_uncertainty_term_with_rectangular_distribution() {
        let term = UncertaintyTerm {
            name: String::from("Rectangular Term"),
            description: String::from("A term with a rectangular distribution"),
            value: 42.0,
            distribution: Distribution::Rectangular,
        };
        let expected_divisor = f64::sqrt(3.0f64);
        assert_eq!(term.divisor(), expected_divisor);
    }

    #[test]
    fn test_uncertainty_term_with_triangular_distribution() {
        let term = UncertaintyTerm {
            name: String::from("Triangular Term"),
            description: String::from("A term with a triangular distribution"),
            value: 42.0,
            distribution: Distribution::Triangular,
        };
        let expected_divisor = f64::sqrt(6.0f64);
        assert_eq!(term.divisor(), expected_divisor);
    }
}
