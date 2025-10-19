//! Role-Based Access Control (RBAC) implementation
//! This module provides simple role-based authorization

use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Role {
    User,
    Admin,
    SuperAdmin,
}

#[derive(Debug, Clone)]
pub struct UserRoles {
    roles: HashMap<String, Role>,
}

impl UserRoles {
    pub fn new() -> Self {
        let mut roles = HashMap::new();
        // In a real application, these would be loaded from a database
        roles.insert("1".to_string(), Role::Admin); // Alice is an admin
        roles.insert("2".to_string(), Role::User);  // Bob is a user
        roles.insert("3".to_string(), Role::User);  // Charlie is a user
        
        Self { roles }
    }
    
    pub fn get_role(&self, user_id: &str) -> Option<&Role> {
        self.roles.get(user_id)
    }
    
    pub fn has_permission(&self, user_id: &str, required_role: &Role) -> bool {
        if let Some(user_role) = self.get_role(user_id) {
            match (user_role, required_role) {
                (Role::SuperAdmin, _) => true,
                (Role::Admin, Role::User) => true,
                (role, required) => role == required,
            }
        } else {
            false
        }
    }
}

pub struct RbacService {
    user_roles: UserRoles,
}

impl RbacService {
    pub fn new() -> Self {
        Self {
            user_roles: UserRoles::new(),
        }
    }
    
    pub fn require_role(&self, user_id: &str, role: &Role) -> Result<(), StatusCode> {
        if self.user_roles.has_permission(user_id, role) {
            Ok(())
        } else {
            Err(StatusCode::FORBIDDEN)
        }
    }
}

// Middleware to check for admin role
pub async fn admin_middleware(
    headers: HeaderMap,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // In a real implementation, you would extract the user ID from the token
    // For this example, we'll hardcode a user ID for demo purposes
    // In a production system, you would validate the JWT token and extract the user ID
    let user_id = "1"; // Alice's ID (admin user from our seed data)
    
    let rbac = RbacService::new();
    rbac.require_role(user_id, &Role::Admin)?;
    
    let response = next.run(req).await;
    Ok(response)
}