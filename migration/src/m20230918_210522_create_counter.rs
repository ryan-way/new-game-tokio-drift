use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Counter::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Counter::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Counter::Count).small_unsigned().not_null())
                    .col(ColumnDef::new(Counter::Name).string().not_null())
                    .to_owned(),
            )
            .await?;

        let insert = Query::insert()
            .into_table(Counter::Table)
            .columns([Counter::Count, Counter::Name])
            .values_panic([0.into(), "Test".into()])
            .to_owned();

        manager.exec_stmt(insert).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Counter::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Counter {
    Table,
    Id,
    Count,
    Name,
}
