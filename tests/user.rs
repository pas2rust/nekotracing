use kenzu::Builder;
use nekotracing::nekotracing;

#[derive(Debug, Builder, Clone)]
pub struct User {
    id: u128,
    name: String,
    age: u8,
}

impl User {
    #[nekotracing]
    fn sync_user(self) -> Result<Self, String> {
        Ok(self
            .name(UserName::new("sync user")?)
            .age(UserAge::new(18)?)
            .id(UserId::new(0)?))
    }
    #[nekotracing]
    async fn async_user(self) -> Result<Self, String> {
        Ok(self
            .name(UserName::new("async user")?)
            .age(UserAge::new(19)?)
            .id(UserId::new(1)?))
    }
}

#[test]
fn sync_user() -> Result<(), String> {
    User::new().sync_user()?;
    Ok(())
}

#[tokio::test]
async fn async_user() -> Result<(), String> {
    User::new().async_user().await?;
    Ok(())
}
