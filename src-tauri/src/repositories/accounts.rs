use crate::models::account::{Account, AccountDto, NewAccount};
use sqlx::postgres::PgPool;
use uuid::Uuid;

pub struct AccountRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> AccountRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_all(&self) -> Result<Vec<Account>, sqlx::Error> {
        let dtos = sqlx::query_as::<_, AccountDto>("SELECT * FROM accounts ORDER BY code")
            .fetch_all(self.pool)
            .await?;

        Ok(dtos.into_iter().map(Account::from).collect())
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Account>, sqlx::Error> {
        let dto = sqlx::query_as::<_, AccountDto>("SELECT * FROM accounts WHERE id = $1")
            .bind(id)
            .fetch_optional(self.pool)
            .await?;

        Ok(dto.map(Account::from))
    }

    pub async fn find_by_code(&self, code: &str) -> Result<Option<Account>, sqlx::Error> {
        let dto = sqlx::query_as::<_, AccountDto>("SELECT * FROM accounts WHERE code = $1")
            .bind(code)
            .fetch_optional(self.pool)
            .await?;

        Ok(dto.map(Account::from))
    }

    pub async fn create(&self, new_account: NewAccount) -> Result<Account, sqlx::Error> {
        let account = Account::new(new_account);
        let dto = AccountDto::from(account.clone());

        sqlx::query(
            r#"
            INSERT INTO accounts
                (id, code, name, description, account_type, category, subcategory, 
                is_active, parent_id, balance, created_at, updated_at)
            VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            "#,
        )
        .bind(dto.id)
        .bind(dto.code)
        .bind(dto.name)
        .bind(dto.description)
        .bind(dto.account_type)
        .bind(dto.category)
        .bind(dto.subcategory)
        .bind(dto.is_active)
        .bind(dto.parent_id)
        .bind(dto.balance)
        .bind(dto.created_at)
        .bind(dto.updated_at)
        .execute(self.pool)
        .await?;

        Ok(account)
    }

    pub async fn update(&self, account: &Account) -> Result<(), sqlx::Error> {
        let dto = AccountDto::from(account.clone());

        sqlx::query(
            r#"
            UPDATE accounts
            SET 
                code = $2,
                name = $3,
                description = $4,
                account_type = $5,
                category = $6,
                subcategory = $7,
                is_active = $8,
                parent_id = $9,
                balance = $10,
                updated_at = $11
            WHERE id = $1
            "#,
        )
        .bind(dto.id)
        .bind(dto.code)
        .bind(dto.name)
        .bind(dto.description)
        .bind(dto.account_type)
        .bind(dto.category)
        .bind(dto.subcategory)
        .bind(dto.is_active)
        .bind(dto.parent_id)
        .bind(dto.balance)
        .bind(dto.updated_at)
        .execute(self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM accounts WHERE id = $1")
            .bind(id)
            .execute(self.pool)
            .await?;

        Ok(())
    }

    pub async fn find_children(&self, parent_id: Uuid) -> Result<Vec<Account>, sqlx::Error> {
        let dtos = sqlx::query_as::<_, AccountDto>(
            "SELECT * FROM accounts WHERE parent_id = $1 ORDER BY code",
        )
        .bind(parent_id)
        .fetch_all(self.pool)
        .await?;

        Ok(dtos.into_iter().map(Account::from).collect())
    }

    pub async fn find_roots(&self) -> Result<Vec<Account>, sqlx::Error> {
        let dtos = sqlx::query_as::<_, AccountDto>(
            "SELECT * FROM accounts WHERE parent_id IS NULL ORDER BY code",
        )
        .fetch_all(self.pool)
        .await?;

        Ok(dtos.into_iter().map(Account::from).collect())
    }

    pub async fn update_balance(
        &self,
        id: Uuid,
        amount: rust_decimal::Decimal,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE accounts
            SET balance = balance + $2, updated_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(id)
        .bind(amount)
        .execute(self.pool)
        .await?;

        Ok(())
    }
}
