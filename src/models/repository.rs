use chrono::NaiveDateTime;
use diesel::{Queryable, Insertable, prelude::*, result::{Error::{NotFound, DatabaseError}, DatabaseErrorKind::UniqueViolation}};
use juniper::graphql_object;
use nanoid::nanoid;

use crate::{schemas::root::Context, validation_result, schema::repositories, db::DBPooledConnection};

use super::user::{UserOperation, UserResult};

#[derive(Queryable)]
pub struct Repository {
    pub id: String,
    pub user_id: String,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    // id          char(21) primary key,
    // user_id     char(21) not null references users(id),
    // slug        text unique not null,
    // name        text not null,
    // description text,
    // created_at  timestamp not null default now(),
    // updated_at  timestamp not null default now()
}

#[graphql_object(
    name = "Repository",
    description = "Repository model",
    context = Context
)]
impl Repository {
    #[graphql(description = "The repositories ID in base64 format")]
    fn id(&self) -> &str {
        &self.id
    }

    #[graphql(description = "The user who owns the repository")]
    fn user(&self, context: &Context) -> UserResult {
        let mut conn = context.db_pool.get().unwrap();
        UserOperation::find(&mut conn, &self.user_id)
    }

    #[graphql(description = "")]
    fn name(&self) -> &str {
        &self.name
    }

    #[graphql(description = "")]
    fn slug(&self) -> &str {
        &self.slug
    }

    #[graphql(description = "")]
    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    #[graphql(description = "DateTime for when the user was created")]
    fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }

    #[graphql(description = "DateTime for when the user was last updated")]
    fn updated_at(&self) -> &NaiveDateTime {
        &self.updated_at
    }
}


validation_result!(RepositoryResult, Repository);

#[derive(Insertable)]
#[diesel(table_name = repositories)]
pub struct NewRepository {
    pub id: String,
    pub user_id: String,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
}

impl NewRepository {
    pub fn new(user_id: &str, slug: &str, name: &str, description: Option<&str>) -> NewRepository {
        NewRepository { 
            id: nanoid!(), 
            user_id: user_id.into(), 
            slug: slug.into(), 
            name: name.into(), 
            description: description.map(|val| val.to_string())
        }
    }
}

pub struct RepositoryOperation;

impl RepositoryOperation {
    pub fn create(conn: &mut DBPooledConnection, user_id: &str, slug: &str, name: &str, description: Option<&str>) -> RepositoryResult {
        let new_repository = NewRepository::new(user_id, slug, name, description);
 
        let result = diesel::insert_into(repositories::table)
            .values(&new_repository)
            .get_result::<Repository>(conn);


        match result {
            Ok(repository) => RepositoryResult::Repository(repository),
            Err(DatabaseError(UniqueViolation, _)) =>
                RepositoryResult::conflict("user already exists"),
            Err(_) => RepositoryResult::server(),
        }
    }

    pub fn find(conn: &mut DBPooledConnection, id: &str) -> RepositoryResult {
        use crate::schema::repositories::dsl::{repositories, id as repository_id};
        
        let repository = repositories
            .filter(repository_id.eq(id))
            .get_result::<Repository>(conn);

        match repository {
            Ok(repository) => RepositoryResult::Repository(repository),
            Err(NotFound) => RepositoryResult::not_found("user not found"),
            Err(_) => RepositoryResult::server(),
        }
    }

    pub fn find_by_user(conn: &mut DBPooledConnection, user_id: &str) -> Vec<Repository> {
        use crate::schema::repositories::dsl::{repositories, user_id as repository_user_id};
        
        let repository_list = repositories
            .filter(repository_user_id.eq(user_id))
            .get_results::<Repository>(conn);

        match repository_list {
            Ok(vals) => vals,
            Err(_) => vec![],
        }
    }
}

