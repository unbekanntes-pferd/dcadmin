use keyring::Entry;

pub trait HandleCredentials {
    fn set_dracoon_env(&self, secret: &str) -> Result<(), String>;
    fn get_dracoon_env(&self) -> Result<String, String>;
    #[allow(unused)]
    fn delete_dracoon_env(&self) -> Result<(), String>;
}

impl HandleCredentials for Entry {
    fn set_dracoon_env(&self, secret: &str) -> Result<(), String> {
        match self.set_password(secret) {
            Ok(()) => Ok(()),
            Err(_) => Err("CredentialStorageFailed".to_string()),
        }
    }
    fn get_dracoon_env(&self) -> Result<String, String> {
        match self.get_password() {
            Ok(pwd) => Ok(pwd),
            Err(_) => Err("InvalidAccount".to_string()),
        }
    }
    fn delete_dracoon_env(&self) -> Result<(), String> {
        if self.get_password().is_err() {
            return Err("InvalidAccount".to_string());
        }

        match self.delete_password() {
            Ok(()) => Ok(()),
            Err(_) => Err("CredentialDeletionFailed".to_string()),
        }
    }
}