macro_rules! select_model {
    ($struct:ty, $table_name:ident) => {
        #[async_trait]
        impl SelectModel for $struct {
            async fn from_id(conn: DbPool, id: i32) -> Self {
                use crate::schema::$table_name::dsl as ($table_name)_table;

                let mut conn = conn.get().unwrap();

                web::block(move || {
                    // Get the Required Resource
                    let selected_model: Vec<$struct> = ($table_name)_table::app_organizations
                        .filter(($table_name)_table::id.eq(id))
                        .load(&mut conn)
                        .unwrap();

                    let selected_model = selected_model.get(0).unwrap();

                    selected_model.clone()
                })
                .await
                .unwrap()
            }
        }
    };
}

