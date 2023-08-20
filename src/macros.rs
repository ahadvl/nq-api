
#[macro_export]
macro_rules! select_model {
    ($struct:ty, $table_name:ident) => {

        #[async_trait]
        impl SelectModel for $struct {
            async fn from_id(conn: DbPool, id: i32) -> Self {
                use crate::schema::$table_name::dsl as $table_name;

                let mut conn = conn.get().unwrap();

                block(move || {
                    // Get the Required Resource
                    let selected_model: Vec<$struct> = $table_name::$table_name
                        .filter($table_name::id.eq(id))
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

