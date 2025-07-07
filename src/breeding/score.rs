#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Score {
    pub primary: f64,
    pub secondary: f64,
    pub tertiary: f64,
}

impl Score {
    pub fn new(primary: f64, secondary: f64, tertiary: f64) -> Self {
        Self {
            primary,
            secondary,
            tertiary,
        }
    }

    pub fn primary(primary: f64) -> Self {
        Self::new(primary, 0.0, 0.0)
    }

    pub fn primary_secondary(primary: f64, secondary: f64) -> Self {
        Self::new(primary, secondary, 0.0)
    }

    pub fn one_number(&self) -> f64 {
        self.primary + self.secondary * 0.01 + self.tertiary * 0.0001
    }
}

impl Eq for Score {}

impl std::cmp::PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Score {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        if self.primary < other.primary {
            Ordering::Less
        } else if self.primary > other.primary {
            Ordering::Greater
        } else if self.secondary < other.secondary {
            Ordering::Less
        } else if self.secondary > other.secondary {
            Ordering::Greater
        } else if self.tertiary < other.tertiary {
            Ordering::Less
        } else if self.tertiary > other.tertiary {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl std::fmt::Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.secondary == 0.0 && self.tertiary == 0.0 {
            write!(f, "{}", self.primary)
        } else if self.tertiary == 0.0 {
            write!(f, "{}.{}", self.primary, self.secondary)
        } else {
            write!(f, "{}.{}.{}", self.primary, self.secondary, self.tertiary)
        }
    }
}
