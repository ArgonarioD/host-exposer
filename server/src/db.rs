use std::path::Path;

use sea_orm::{DbBackend, DbConn, Schema};
use sea_orm::prelude::*;

use crate::entity::prelude::DbClient;

pub async fn setup_schema(db: &DbConn) {
    if Path::new("data.sqlite").exists() {
        return;
    }
    let schema = Schema::new(DbBackend::Sqlite);
    let mut stmt = schema.create_table_from_entity(DbClient);
    let stmt = stmt.if_not_exists();
    db.execute(db.get_database_backend().build(stmt)).await.unwrap();
}

pub(crate) mod client {
    use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, QueryFilter};
    use sea_orm::ActiveValue::Set;
    use sea_orm::prelude::Expr;
    use time::OffsetDateTime;
    use uuid::Uuid;

    use crate::entity::client;
    use crate::entity::prelude::DbClient;
    use crate::result::HEError;

    pub async fn save_new_client_information(id: &Uuid, db: &DbConn) -> Result<(), HEError> {
        let db_client = DbClient::find_by_id(*id).one(db).await?;
        if db_client.is_none() {
            let now = OffsetDateTime::now_local().unwrap();
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

    pub async fn update_clients_fetch_time(ids: &[Uuid], db: &DbConn) -> Result<(), HEError> {
        DbClient::update_many()
            .col_expr(client::Column::LastFetchTime, Expr::value(OffsetDateTime::now_local().unwrap()))
            .filter(client::Column::Id.is_in(ids.iter().copied()))
            .exec(db)
            .await?;
        Ok(())
    }

    pub async fn modify_client_name(id: &Uuid, new_name: String, db: &DbConn) -> Result<(), HEError> {
        let db_client = DbClient::find_by_id(*id).one(db).await?;
        let mut db_client: client::ActiveModel = db_client.unwrap().into();
        db_client.name = Set(new_name);
        db_client.update(db).await?;
        Ok(())
    }
}