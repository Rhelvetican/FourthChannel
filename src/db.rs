use std::{
    ops::{Deref, DerefMut},
    path::Path,
};

use rusqlite::{params, Connection};
use sea_query::{ColumnDef, Iden, SqliteQueryBuilder, Table};

use crate::util::Res;

pub(crate) trait DatabaseProvider {
    fn get_connection(self) -> Res<Connection>;
}

pub struct InMemProvider;

impl DatabaseProvider for InMemProvider {
    fn get_connection(self) -> Res<Connection> {
        Connection::open_in_memory().map_err(|e| e.into())
    }
}

pub struct InFileProvider<P: AsRef<Path>>(P);

impl<P: AsRef<Path>> InFileProvider<P> {
    pub fn new(path: P) -> Self {
        InFileProvider(path)
    }
}

impl<P: AsRef<Path>> DatabaseProvider for InFileProvider<P> {
    fn get_connection(self) -> Res<Connection> {
        let InFileProvider(path) = self;

        Connection::open(path).map_err(|e| e.into())
    }
}

#[derive(Debug, Iden, Clone, Copy)]
pub enum Posts {
    // The Table
    Table,
    // Content
    Content,
    // Ratings
    Rating,
    // Ratings
    Hash,
    // Ratings
    Media,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn from_provider<Provider: DatabaseProvider>(provider: Provider) -> Res<Self> {
        Ok(Self {
            conn: provider.get_connection()?,
        })
    }

    pub fn migrations(&self) -> Res<()> {
        let migrations = Table::create()
            .if_not_exists()
            .table(Posts::Table)
            .col(ColumnDef::new(Posts::Rating).integer().not_null())
            .col(ColumnDef::new(Posts::Content).text())
            .col(ColumnDef::new(Posts::Media).blob())
            .col(ColumnDef::new(Posts::Hash).string().not_null())
            .build(SqliteQueryBuilder);

        self.conn.execute(&migrations, params![])?;

        Ok(())
    }
}

impl Deref for Database {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        &self.conn
    }
}

impl DerefMut for Database {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.conn
    }
}
