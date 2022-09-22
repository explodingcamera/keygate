use crate::{models, KeySignal};

pub trait Identity: Send + Sync {
    fn identity_get(&self, user_id: &str) -> Result<models::Identity, String>;
    fn identity_delete(&self, user_id: &str) -> Result<(), String>;
    fn identity_update(&self, user_id: &str, identity: &models::Identity) -> Result<(), String>;
    fn identity_update_in_place<F>(
        &self,
        user_id: &str,
        closure: F,
    ) -> Result<Option<models::Identity>, String>
    where
        F: FnOnce(models::Identity) -> Option<models::Identity>;
    fn identities(&self) -> Result<(), String>;
}

impl Identity for KeySignal {
    fn identity_update_in_place<F>(
        &self,
        user_id: &str,
        closure: F,
    ) -> Result<Option<models::Identity>, String>
    where
        F: FnOnce(models::Identity) -> Option<models::Identity>,
    {
        let identity = self.identity_get(user_id)?;
        match closure(identity) {
            Some(new_identity) => {
                self.identity_update(user_id, &new_identity)?;
                Ok(Some(new_identity))
            }
            None => Ok(None),
        }
    }

    fn identity_get(&self, user_id: &str) -> Result<models::Identity, String> {
        todo!()
    }

    fn identity_delete(&self, user_id: &str) -> Result<(), String> {
        todo!()
    }

    fn identity_update(&self, user_id: &str, identity: &models::Identity) -> Result<(), String> {
        todo!()
    }

    fn identities(&self) -> Result<(), String> {
        todo!()
    }
}
