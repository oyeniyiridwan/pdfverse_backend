use sea_orm::{ColumnTrait, QueryFilter};

use crate::database::tasks;
pub trait SoftDeleteFilter :Sized {
     fn not_deleted(self)->Self;
}

impl SoftDeleteFilter for sea_orm::Select<tasks::Entity> {
    fn not_deleted(self)->Self {
        self.filter(tasks::Column::DeletedAt.is_null())
    }
}



