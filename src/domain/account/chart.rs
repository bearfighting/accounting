use super::{account::Account, category::Category};

pub struct Chart {
    pub accounts: Vec<Account>,
}

impl Chart {
    pub fn new() -> Self {
        Chart {
            accounts: Vec::new(),
        }
    }

    pub fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    pub fn add_accounts(&mut self, accounts: Vec<Account>) {
        self.accounts.extend(accounts);
    }

    pub fn get_account_by_id(&self, id: u32) -> Option<&Account> {
        self.accounts.iter().find(|&account| account.r#ref == id)
    }

    pub fn get_accounts_by_category(&self, category: Category) -> Vec<&Account> {
        self.accounts
            .iter()
            .filter(|&account| account.category() == category)
            .collect()
    }
}
