use anyhow::{Result, Context};
use mongodb::{Client, Collection, bson::doc};
use crate::config::Config;
use crate::repository::Commit;

pub struct MongoStore {
    collection: Collection<Commit>,
}

impl MongoStore {
    pub async fn new(config: &Config) -> Result<Self> {
        let client = Client::with_uri_str(&config.mongodb_url)
            .await
            .context("Failed to connect to MongoDB")?;
        
        // Test connection
        client
            .database("admin")
            .run_command(doc! { "ping": 1 })
            .await
            .context("Failed to ping MongoDB")?;
        
        let db = client.database(&config.mongodb_db);
        let collection = db.collection::<Commit>(&config.mongodb_collection);
        
        Ok(Self { collection })
    }
    
    pub async fn save_commits(&self, commits: &[Commit]) -> Result<()> {
        if commits.is_empty() {
            return Ok(());
        }
        
        for commit in commits {
            self.collection
                .replace_one(
                    doc! { "hash": &commit.hash },
                    commit,
                )
                .upsert(true)
                .await?;
        }
        
        Ok(())
    }
    
    #[allow(dead_code)]
    pub async fn get_all_commits(&self) -> Result<Vec<Commit>> {
        let mut cursor = self.collection.find(doc! {}).await?;
        let mut commits = Vec::new();
        
        use futures::stream::StreamExt;
        while let Some(result) = cursor.next().await {
            commits.push(result?);
        }
        
        Ok(commits)
    }
}
