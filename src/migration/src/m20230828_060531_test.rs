use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Test::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Test::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Test::Message).string().not_null())
                    .col(ColumnDef::new(Test::Count).integer().not_null())
                    .to_owned(),
            )
            .await?;

        let insert = Query::insert()
            .into_table(Test::Table)
            .columns([Test::Message, Test::Count])
            .values_panic(["Test Message".into(), 72.into()])
            .to_owned();

        manager.exec_stmt(insert).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Test::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Test {
    Table,
    Id,
    Message,
    Count,
}
