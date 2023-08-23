use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(HelloWorld::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(HelloWorld::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(HelloWorld::Message).string().not_null())
                    .col(
                        ColumnDef::new(HelloWorld::SomethingElse)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        let insert = Query::insert()
            .into_table(HelloWorld::Table)
            .columns([HelloWorld::Message, HelloWorld::SomethingElse])
            .values_panic(["Hello World".into(), "Test Message".into()])
            .to_owned();
        manager.exec_stmt(insert).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(HelloWorld::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum HelloWorld {
    Table,
    Id,
    Message,
    SomethingElse,
}
