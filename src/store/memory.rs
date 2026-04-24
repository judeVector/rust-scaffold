use crate::models::user::User;
use dashmap::DashMap;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct MemoryStore {
    users: Arc<DashMap<Uuid, User>>,
}

impl MemoryStore {
    pub fn new() -> Self {
        Self {
            users: Arc::new(DashMap::new()),
        }
    }

    pub fn create_user(&self, user: User) -> User {
        self.users.insert(user.id, user.clone());
        user
    }

    pub fn get_user(&self, id: &Uuid) -> Option<User> {
        self.users.get(id).map(|r| r.value().clone())
    }

    pub fn get_user_by_email(&self, email: &str) -> Option<User> {
        self.users
            .iter()
            .find(|r| r.value().email == email)
            .map(|r| r.value().clone())
    }

    pub fn list_users(&self) -> Vec<User> {
        let mut users: Vec<User> = self.users.iter().map(|r| r.value().clone()).collect();
        users.sort_by_key(|u| u.created_at);
        users
    }

    pub fn update_user(&self, id: &Uuid, f: impl FnOnce(&mut User)) -> Option<User> {
        self.users.get_mut(id).map(|mut r| {
            f(r.value_mut());
            r.value().clone()
        })
    }

    pub fn delete_user(&self, id: &Uuid) -> bool {
        self.users.remove(id).is_some()
    }

    pub fn email_exists(&self, email: &str) -> bool {
        self.users.iter().any(|r| r.value().email == email)
    }
}
