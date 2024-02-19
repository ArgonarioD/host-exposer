use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Client::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Client::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Client::Name).string().not_null()
                    )
                    .col(
                        ColumnDef::new(Client::CreateTime).date_time().not_null()
                    )
                    .col(
                        ColumnDef::new(Client::LastFetchTime).date_time().not_null()
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Client::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Client {
    Table,
    Id,
    Name,
    CreateTime,
    LastFetchTime,
}
