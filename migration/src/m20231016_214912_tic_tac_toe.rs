use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TicTacToe::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TicTacToe::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TicTacToe::Cell1).string().not_null())
                    .col(ColumnDef::new(TicTacToe::Cell2).string().not_null())
                    .col(ColumnDef::new(TicTacToe::Cell3).string().not_null())
                    .col(ColumnDef::new(TicTacToe::Cell4).string().not_null())
                    .col(ColumnDef::new(TicTacToe::Cell5).string().not_null())
                    .col(ColumnDef::new(TicTacToe::Cell6).string().not_null())
                    .col(ColumnDef::new(TicTacToe::Cell7).string().not_null())
                    .col(ColumnDef::new(TicTacToe::Cell8).string().not_null())
                    .col(ColumnDef::new(TicTacToe::Cell9).string().not_null())
                    .to_owned(),
            )
            .await?;

        let insert = Query::insert()
            .into_table(TicTacToe::Table)
            .columns([
                TicTacToe::Cell1,
                TicTacToe::Cell2,
                TicTacToe::Cell3,
                TicTacToe::Cell4,
                TicTacToe::Cell5,
                TicTacToe::Cell6,
                TicTacToe::Cell7,
                TicTacToe::Cell8,
                TicTacToe::Cell9,
            ])
            .values_panic(["X", " ", "O", "O", " ", " ", " ", "X", "O"].map(|s| s.into()))
            .to_owned();

        manager.exec_stmt(insert).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TicTacToe::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TicTacToe {
    Table,
    Id,
    Cell1,
    Cell2,
    Cell3,
    Cell4,
    Cell5,
    Cell6,
    Cell7,
    Cell8,
    Cell9,
}
