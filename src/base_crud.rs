use sea_orm::sea_query::OnConflict;
use sea_orm::{
    ActiveModelTrait, ConnectionTrait, DbErr, EntityTrait, FromQueryResult, InsertResult,
    IntoActiveModel, ModelTrait, PrimaryKeyTrait,
};

#[async_trait]
pub trait BaseCRUD<Entity, Model, ActiveModel>
where
    Entity: EntityTrait<Model = Model>,
    Model: ModelTrait + FromQueryResult + IntoActiveModel<ActiveModel>,
    ActiveModel: ActiveModelTrait<Entity = Entity> + Send,
{
    async fn get_by_id(
        db: &impl ConnectionTrait,
        id: <<Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType,
    ) -> Result<Option<Model>, DbErr> {
        Entity::find_by_id(id).one(db).await
    }

    async fn create<'a>(
        db: &impl ConnectionTrait,
        active_model: ActiveModel,
    ) -> Result<InsertResult<ActiveModel>, DbErr>
    where
        ActiveModel: 'a,
    {
        Entity::insert(active_model).exec(db).await
    }

    async fn create_with_returning<'a>(
        db: &impl ConnectionTrait,
        active_model: ActiveModel,
    ) -> Result<<ActiveModel::Entity as EntityTrait>::Model, DbErr>
    where
        ActiveModel: 'a,
    {
        Entity::insert(active_model).exec_with_returning(db).await
    }

    async fn create_unique<'a>(
        db: &impl ConnectionTrait,
        active_model: ActiveModel,
    ) -> Result<InsertResult<ActiveModel>, DbErr>
    where
        ActiveModel: 'a,
    {
        Entity::insert(active_model)
            .on_conflict(OnConflict::new().do_nothing().to_owned())
            .exec(db)
            .await
    }

    async fn create_unique_with_returning<'a>(
        db: &impl ConnectionTrait,
        active_model: ActiveModel,
    ) -> Result<<ActiveModel::Entity as EntityTrait>::Model, DbErr>
    where
        ActiveModel: 'a,
    {
        Entity::insert(active_model)
            .on_conflict(OnConflict::new().do_nothing().to_owned())
            .exec_with_returning(db)
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::base_crud::BaseCRUD;
    use sea_orm::ActiveValue::Set;
    use sea_orm::{DatabaseBackend, MockDatabase};

    mod entities {
        use sea_orm::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
        #[sea_orm(table_name = "login_sessions")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = true)]
            pub id: i32,
            pub name: String,
        }

        #[derive(Copy, Clone, Debug, EnumIter)]
        pub enum Relation {}

        impl RelationTrait for Relation {
            fn def(&self) -> RelationDef {
                panic!("No RelationDef")
            }
        }

        impl ActiveModelBehavior for ActiveModel {}
    }

    #[tokio::test]
    async fn test_base_crud() {
        use entities::{self, Entity as Cat};

        struct CatCrud {}
        impl BaseCRUD<Cat, entities::Model, entities::ActiveModel> for CatCrud {}

        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![entities::Model {
                id: 1,
                name: "Boris".into(),
            }]])
            .append_query_results(vec![
                vec![entities::Model {
                    id: 2,
                    name: "Murzik".into(),
                }],
                vec![entities::Model {
                    id: 3,
                    name: "Murka".into(),
                }],
            ])
            .into_connection();

        assert_eq!(
            CatCrud::get_by_id(&db, 1).await.unwrap(),
            Some(entities::Model {
                id: 1,
                name: "Boris".into()
            })
        );

        assert_eq!(
            CatCrud::create(
                &db,
                entities::ActiveModel {
                    name: Set("Murzik".into()),
                    ..Default::default()
                },
            )
            .await
            .unwrap()
            .last_insert_id,
            2
        );

        assert_eq!(
            CatCrud::create_with_returning(
                &db,
                entities::ActiveModel {
                    name: Set("Murka".into()),
                    ..Default::default()
                }
            )
            .await
            .unwrap(),
            entities::Model {
                id: 3,
                name: "Murka".to_string()
            }
        )
    }
}
