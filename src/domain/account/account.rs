use super::category::Category;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    pub r#ref: u32,
    pub name: String,
    pub parent_ref: Option<u32>,
}

impl Account {
    pub fn new(r#ref: u32, name: String) -> Self {
        Account {
            r#ref,
            name,
            parent_ref: Self::parent_ref(r#ref),
        }
    }

    pub fn category(&self) -> Category {
        let id_string = self.r#ref.to_string();

        match id_string.chars().next() {
            Some('1') => Category::Asset,
            Some('2') => Category::Liability,
            Some('3') => Category::Equity,
            Some('4') => Category::Revenue,
            Some('5') => Category::Expense,
            _ => panic!("Unknown category for account ID: {}", self.r#ref),
        }
    }

    pub fn parent_ref(r#ref: u32) -> Option<u32> {
        if r#ref < 10 { None } else { Some(r#ref / 10) }
    }
}
