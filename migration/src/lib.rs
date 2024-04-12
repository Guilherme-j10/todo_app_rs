pub use sea_orm_migration::prelude::*;

mod m20240411_193554_todo_table;
mod m20240412_122421_users;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240411_193554_todo_table::Migration),
            Box::new(m20240412_122421_users::Migration),
        ]
    }
}