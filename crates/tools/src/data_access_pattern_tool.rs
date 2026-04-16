//! Data Access Pattern tool - Repository and EF Core patterns
use avalonia_mcp_core::error::AvaloniaMcpError;
use avalonia_mcp_core::markdown::MarkdownOutputBuilder;
use rmcp::model::{CallToolResult, Content};
use rmcp::tool;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DataAccessPatternParams {
    pub pattern: Option<String>,
    pub include_examples: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AsyncDataAccessParams {
    pub service_name: String,
    pub include_caching: Option<bool>,
    pub include_retry: Option<bool>,
    pub caching_provider: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct DataAccessPatternTool;

impl DataAccessPatternTool {
    pub fn new() -> Self { Self }

    #[tool(description = "Generate data access patterns for AvaloniaUI applications. Covers EF Core, Dapper, repository pattern, and database best practices.")]
    pub async fn generate_data_access_pattern(
        &self,
        params: DataAccessPatternParams,
    ) -> Result<CallToolResult, AvaloniaMcpError> {
        let include_examples = params.include_examples.unwrap_or(true);
        let pattern = params.pattern.as_deref().unwrap_or("efcore");

        let output = match pattern {
            "efcore" => self.generate_efcore(include_examples),
            "dapper" => self.generate_dapper(include_examples),
            "repository" => self.generate_repository(include_examples),
            _ => self.generate_efcore(include_examples),
        };

        Ok(CallToolResult::success(vec![Content::text(output)]))
    }

    #[tool(description = "Creates async data access patterns with caching and error handling for AvaloniaUI applications")]
    pub async fn generate_async_data_access(
        &self,
        params: AsyncDataAccessParams,
    ) -> Result<CallToolResult, AvaloniaMcpError> {
        if params.service_name.is_empty() {
            return Err(AvaloniaMcpError::validation("Service name cannot be empty"));
        }

        let include_caching = params.include_caching.unwrap_or(true);
        let include_retry = params.include_retry.unwrap_or(true);
        let cache_provider = params.caching_provider.as_deref().unwrap_or("memory");

        let table_name = params.service_name.replace("Service", "").to_lowercase();

        let mut builder = MarkdownOutputBuilder::new()
            .heading(1, &format!("Async Data Access Service: {}", params.service_name))
            .heading(2, "Configuration")
            .task_list(vec![
                (true, format!("Service: {}", params.service_name)),
                (true, format!("Caching: {}", include_caching)),
                (true, format!("Retry: {}", include_retry)),
            ])
            .heading(2, "Service Interface")
            .code_block(
                "csharp",
                &format!(
                    "public interface I{service}\n{{\n    Task<T?> GetDataAsync<T>(string key);\n    Task SaveDataAsync<T>(string key, T data);\n}}",
                    service = params.service_name
                ),
            )
            .heading(2, "Async Implementation")
            .code_block(
                "csharp",
                &format!(
                    r#"public class {service} : I{service}
{{
    private readonly IDbConnection _connection;

    public async Task<T?> GetDataAsync<T>(string key)
    {{
        return await _connection.QueryFirstOrDefaultAsync<T>(
            "SELECT * FROM {table} WHERE Key = @Key", new {{ Key = key }});
    }}

    public async Task SaveDataAsync<T>(string key, T data)
    {{
        await _connection.ExecuteAsync(
            "INSERT OR REPLACE INTO {table} (Key, Data) VALUES (@Key, @Data)",
            new {{ Key = key, Data = JsonSerializer.Serialize(data) }});
    }}
}}"#,
                    service = params.service_name,
                    table = table_name
                ),
            );

        if include_caching {
            builder = builder
                .heading(2, "Caching Implementation")
                .code_block(
                    "csharp",
                    &format!(
                        r#"// {provider} caching implementation
private readonly IMemoryCache _cache;
private readonly TimeSpan _cacheDuration = TimeSpan.FromMinutes(5);

public async Task<T?> GetCachedAsync<T>(string key, Func<Task<T>> factory)
{{
    return await _cache.GetOrCreateAsync(key, async entry =>
    {{
        entry.AbsoluteExpirationRelativeToNow = _cacheDuration;
        return await factory();
    }});
}}"#,
                        provider = cache_provider
                    ),
                );
        }

        if include_retry {
            builder = builder
                .heading(2, "Retry Policy")
                .code_block(
                    "csharp",
                    r#"// Retry policy with exponential backoff
public async Task<T> ExecuteWithRetryAsync<T>(Func<Task<T>> operation, int maxRetries = 3)
{
    for (int attempt = 1; attempt <= maxRetries; attempt++)
    {
        try { return await operation(); }
        catch (Exception ex) when (attempt < maxRetries)
        {
            await Task.Delay(TimeSpan.FromMilliseconds(100 * Math.Pow(2, attempt)));
        }
    }
    throw new Exception("Max retries exceeded");
}"#,
                );
        }

        builder = builder
            .heading(2, "Performance Considerations")
            .list(vec![
                "All DB operations are async",
                "Use cancellation tokens",
                "Connection pooling enabled",
                "Query optimization recommended",
            ]);

        Ok(CallToolResult::success(vec![Content::text(builder.build())]))
    }

    fn generate_efcore(&self, include_examples: bool) -> String {
        let mut builder = MarkdownOutputBuilder::new()
            .heading(1, "Entity Framework Core Pattern")
            .paragraph("EF Core ORM for database access in AvaloniaUI applications.")
            .heading(2, "DbContext Setup")
            .code_block("csharp", r#"public class AppDbContext : DbContext
{
    public AppDbContext(DbContextOptions<AppDbContext> options)
        : base(options) { }
    
    public DbSet<User> Users => Set<User>();
    public DbSet<Order> Orders => Set<Order>();
    
    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        base.OnModelCreating(modelBuilder);
        
        // Configure entities
        modelBuilder.Entity<User>(entity =>
        {
            entity.HasKey(e => e.Id);
            entity.Property(e => e.Email).IsRequired().HasMaxLength(256);
            entity.HasIndex(e => e.Email).IsUnique();
        });
    }
}

// Entity
public class User
{
    public int Id { get; set; }
    public string Name { get; set; } = "";
    public string Email { get; set; } = "";
    public DateTime CreatedAt { get; set; } = DateTime.UtcNow;
}"#);

        if include_examples {
            builder = builder
                .heading(2, "DI Registration")
                .code_block("csharp", r#"// Register DbContext
services.AddDbContext<AppDbContext>(options =>
    options.UseSqlite("Data Source=app.db"));

// Register for ViewModel injection
services.AddScoped<IUserService, UserService>();"#)
                .heading(2, "Async Queries")
                .code_block("csharp", r#"// In your service
public class UserService
{
    private readonly AppDbContext _context;
    
    public UserService(AppDbContext context) => _context = context;
    
    public async Task<User?> GetByIdAsync(int id) =>
        await _context.Users.FindAsync(id);
    
    public async Task<List<User>> GetAllAsync() =>
        await _context.Users.ToListAsync();
    
    public async Task<User> CreateAsync(User user)
    {
        _context.Users.Add(user);
        await _context.SaveChangesAsync();
        return user;
    }
    
    // Efficient loading
    public async Task<User?> WithOrdersAsync(int id) =>
        await _context.Users
            .Include(u => u.Orders)
            .FirstOrDefaultAsync(u => u.Id == id);
}"#);
        }

        builder.heading(2, "Best Practices")
            .task_list(vec![(true, "Use async methods"), (true, "Enable sensitive data logging in dev"), (true, "Use migrations for schema changes"), (true, "Implement soft delete"), (false, "Add retry policies for transient errors")])
            .build()
    }

    fn generate_dapper(&self, _include_examples: bool) -> String {
        MarkdownOutputBuilder::new()
            .heading(1, "Dapper Micro-ORM")
            .paragraph("Lightweight database access with Dapper.")
            .heading(2, "Setup")
            .code_block("csharp", r#"// Install: dotnet add package Dapper

public class UserRepository
{
    private readonly IDbConnection _db;
    
    public UserRepository(IDbConnection db) => _db = db;
    
    public async Task<User?> GetByIdAsync(int id) =>
        await _db.QueryFirstOrDefaultAsync<User>(
            "SELECT * FROM Users WHERE Id = @Id", 
            new { Id = id });
    
    public async Task<IEnumerable<User>> GetAllAsync() =>
        await _db.QueryAsync<User>("SELECT * FROM Users");
    
    public async Task<int> CreateAsync(User user) =>
        await _db.ExecuteAsync(
            "INSERT INTO Users (Name, Email) VALUES (@Name, @Email)",
            user);
}"#)
            .build()
    }

    fn generate_repository(&self, _include_examples: bool) -> String {
        MarkdownOutputBuilder::new()
            .heading(1, "Repository Pattern")
            .paragraph("Abstract data access behind repository interface.")
            .heading(2, "Generic Repository")
            .code_block("csharp", r#"public interface IRepository<T> where T : class
{
    Task<IEnumerable<T>> GetAllAsync();
    Task<T?> GetByIdAsync(int id);
    Task<T> AddAsync(T entity);
    Task DeleteAsync(int id);
}

public class EfRepository<T> : IRepository<T> where T : class
{
    protected readonly DbContext Context;
    protected readonly DbSet<T> DbSet;
    
    public EfRepository(DbContext context)
    {
        Context = context;
        DbSet = context.Set<T>();
    }
    
    public virtual async Task<IEnumerable<T>> GetAllAsync() =>
        await DbSet.ToListAsync();
    
    public virtual async Task<T?> GetByIdAsync(int id) =>
        await DbSet.FindAsync(id);
    
    public virtual async Task<T> AddAsync(T entity)
    {
        await DbSet.AddAsync(entity);
        await Context.SaveChangesAsync();
        return entity;
    }
    
    public virtual async Task DeleteAsync(int id)
    {
        var entity = await DbSet.FindAsync(id);
        if (entity != null)
        {
            DbSet.Remove(entity);
            await Context.SaveChangesAsync();
        }
    }
}"#)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_generate_data_access() {
        let tool = DataAccessPatternTool::new();
        let result = tool.generate_data_access_pattern(DataAccessPatternParams { pattern: None, include_examples: Some(true) }).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_async_data_access_success() {
        let tool = DataAccessPatternTool::new();
        let params = AsyncDataAccessParams {
            service_name: "UserService".to_string(),
            include_caching: Some(true),
            include_retry: Some(true),
            caching_provider: Some("redis".to_string()),
        };
        let result = tool.generate_async_data_access(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
        assert!(result.content.len() > 0);
    }

    #[tokio::test]
    async fn test_generate_async_data_access_no_caching() {
        let tool = DataAccessPatternTool::new();
        let params = AsyncDataAccessParams {
            service_name: "OrderService".to_string(),
            include_caching: Some(false),
            include_retry: Some(false),
            caching_provider: None,
        };
        let result = tool.generate_async_data_access(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_async_data_access_empty_name() {
        let tool = DataAccessPatternTool::new();
        let params = AsyncDataAccessParams {
            service_name: String::new(),
            include_caching: None,
            include_retry: None,
            caching_provider: None,
        };
        let result = tool.generate_async_data_access(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_generate_async_data_access_defaults() {
        let tool = DataAccessPatternTool::new();
        let params = AsyncDataAccessParams {
            service_name: "DataAccess".to_string(),
            include_caching: None,
            include_retry: None,
            caching_provider: None,
        };
        let result = tool.generate_async_data_access(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }
}
