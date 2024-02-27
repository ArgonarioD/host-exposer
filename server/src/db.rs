use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use tracing::log;

use crate::migration::Migrator;
use crate::result::HEError;

pub async fn setup_db_connection() -> Result<DatabaseConnection, HEError> {
    let mut opt = ConnectOptions::new("sqlite://data.sqlite?mode=rwc".to_owned());
    opt.sqlx_logging_level(log::LevelFilter::Debug);
    let db = Database::connect(opt).await?;
    Migrator::up(&db, None).await?;
    Ok(db)
}

pub(crate) mod client {
    use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
    use sea_orm::ActiveValue::Set;
    use sea_orm::prelude::Expr;
    use time::UtcOffset;
    use uuid::Uuid;

    use crate::entity::client;
    use crate::entity::prelude::DbClient;
    use crate::result::HEError;
    use crate::times::local_offset_date_time;

    pub async fn save_new_client_information(id: &Uuid, db: &DatabaseConnection, default_offset: &UtcOffset) -> Result<(), HEError> {
        let db_client = DbClient::find_by_id(*id).one(db).await?;
        if db_client.is_none() {
            let now = local_offset_date_time(default_offset);
            let new_client = client::ActiveModel {
                id: Set(*id),
                name: Set(id.to_string()),
                create_time: Set(now),
                last_fetch_time: Set(now),
            };
            if let Err(db_err) = new_client.insert(db).await {
                return Err(HEError::Db(db_err));
            }
        }
        Ok(())
    }

    pub async fn update_clients_fetch_time(ids: &[Uuid], db: &DatabaseConnection, default_offset: &UtcOffset) -> Result<(), HEError> {
        DbClient::update_many()
            .col_expr(client::Column::LastFetchTime, Expr::value(local_offset_date_time(default_offset)))
            .filter(client::Column::Id.is_in(ids.iter().copied()))
            .exec(db)
            .await?;
        Ok(())
    }

    pub async fn modify_client_name(id: &Uuid, new_name: String, db: &DatabaseConnection) -> Result<(), HEError> {
        let db_client = DbClient::find_by_id(*id).one(db).await?;
        let mut db_client: client::ActiveModel = db_client.unwrap().into();
        db_client.name = Set(new_name);
        db_client.update(db).await?;
        Ok(())
    }
}