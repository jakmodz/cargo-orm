use std::error::Error;



trait Repository : Sized + Sync {
    type PrimaryKey;
    
    async fn save(&self) -> Result<(),  Box<dyn Error>>;
    async fn find()->Result<Self, Box<dyn Error>>;
    async fn get_all()->Result<Vec<Self>, Box<dyn Error>>;
    async fn get_byt_id(id: Self::PrimaryKey)->Result<Self, Box<dyn Error>>;
}