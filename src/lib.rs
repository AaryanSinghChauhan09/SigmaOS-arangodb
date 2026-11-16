//! SigmaOS-arangodb: ArangoDB database model.
//!
//! Real implementation: databases, collections, validation.
//! Replaces the 31-line stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Database {
    pub id: String,
    pub name: String,
    pub replication: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Collection {
    pub id: String,
    pub database_id: String,
    pub shard_key: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArangodbError {
    EmptyDatabaseName,
    DuplicateDatabase(String),
}

impl fmt::Display for ArangodbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyDatabaseName => write!(f, "database name must not be empty"),
            Self::DuplicateDatabase(id) => write!(f, "duplicate database {:?}", id),
        }
    }
}

impl std::error::Error for ArangodbError {}

pub struct Arangodb {
    pub name: String,
    databases: BTreeMap<String, Database>,
    collections: BTreeMap<String, Collection>,
}

impl Arangodb {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), databases: BTreeMap::new(), collections: BTreeMap::new() }
    }

    pub fn add_database(&mut self, d: Database) -> Result<(), ArangodbError> {
        if d.name.trim().is_empty() { return Err(ArangodbError::EmptyDatabaseName); }
        let id = d.id.clone();
        if self.databases.contains_key(&id) {
            return Err(ArangodbError::DuplicateDatabase(id.clone()));
        }
        self.databases.insert(id, d);
        Ok(())
    }

    pub fn database_count(&self) -> usize { self.databases.len() }

    pub fn get_database(&self, id: &str) -> Option<&Database> { self.databases.get(id) }

    pub fn add_collection(&mut self, c: Collection) {
        self.collections.insert(c.id.clone(), c);
    }

    pub fn get_collection(&self, id: &str) -> Option<&Collection> { self.collections.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_aaa() {
        let a = Arangodb::new("Arangodb");
        assert_eq!(a.name, "Arangodb");
        assert_eq!(a.database_count(), 0);
    }

    #[test]
    fn test_add_aaa() {
        let mut a = Arangodb::new("Arangodb");
        let db = Database { id: "d1".into(), name: "mydb".into(), replication: "3".into() };
        a.add_database(db).unwrap();
        assert_eq!(a.database_count(), 1);
    }

    #[test]
    fn test_empty_rejected_aaa() {
        let mut a = Arangodb::new("Arangodb");
        let db = Database { id: "x".into(), name: "".into(), replication: "x".into() };
        assert!(a.add_database(db).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut a = Arangodb::new("Arangodb");
        let db = Database { id: "x".into(), name: "X".into(), replication: "x".into() };
        a.add_database(db).unwrap();
        let db2 = Database { id: "x".into(), name: "X".into(), replication: "x".into() };
        assert!(a.add_database(db2).is_err());
    }
}
