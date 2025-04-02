//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.8

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "PreDownInfoBean")]
pub struct Model {
    pub task_id: i32,
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub pkg_name: String,
    #[sea_orm(column_type = "Text")]
    pub md5: String,
    #[sea_orm(column_type = "Text")]
    pub pre_url: String,
    #[sea_orm(column_type = "Text")]
    pub path: String,
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub file_name: String,
    pub enable: i32,
    pub state: i32,
    pub check_update_time: i32,
    pub start_time: i32,
    pub end_time: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
