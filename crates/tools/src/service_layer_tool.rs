//! Service Layer tool - Service patterns and implementation
use avalonia_mcp_core::error::AvaloniaMcpError;
use avalonia_mcp_core::markdown::MarkdownOutputBuilder;
use rmcp::model::{CallToolResult, Content};
use rmcp::tool;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ServiceLayerParams {
    pub service_type: Option<String>,
    pub include_examples: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DomainServiceParams {
    pub domain_name: String,
    pub include_validation: Option<bool>,
    pub include_events: Option<bool>,
}

#[derive(Debug, Clone, Default)]
pub struct ServiceLayerTool;

impl ServiceLayerTool {
    pub fn new() -> Self { Self }

    #[tool(description = "Generate service layer patterns for AvaloniaUI applications. Covers service interfaces, dependency injection, and business logic organization.")]
    pub async fn generate_service_layer(
        &self,
        params: ServiceLayerParams,
    ) -> Result<CallToolResult, AvaloniaMcpError> {
        let include_examples = params.include_examples.unwrap_or(true);
        let service_type = params.service_type.as_deref().unwrap_or("general");

        let output = match service_type {
            "repository" => self.generate_repository_pattern(include_examples),
            "unitofwork" => self.generate_unit_of_work(include_examples),
            "mediator" => self.generate_mediat_r_pattern(include_examples),
            _ => self.generate_general_services(include_examples),
        };

        Ok(CallToolResult::success(vec![Content::text(output)]))
    }

    fn generate_general_services(&self, include_examples: bool) -> String {
        let mut builder = MarkdownOutputBuilder::new()
            .heading(1, "Service Layer Patterns")
            .paragraph("Service layer implementation for AvaloniaUI applications.")
            .heading(2, "Service Interface")
            .code_block("csharp", r#"public interface IDataService
{
    Task<IEnumerable<Item>> GetAllAsync(CancellationToken ct = default);
    Task<Item?> GetByIdAsync(int id, CancellationToken ct = default);
    Task<Item> CreateAsync(Item item, CancellationToken ct = default);
    Task UpdateAsync(Item item, CancellationToken ct = default);
    Task DeleteAsync(int id, CancellationToken ct = default);
}

public class DataService : IDataService
{
    private readonly IRepository _repository;
    private readonly IMapper _mapper;
    
    public DataService(IRepository repository, IMapper mapper)
    {
        _repository = repository;
        _mapper = mapper;
    }
    
    public async Task<IEnumerable<Item>> GetAllAsync(CancellationToken ct = default)
    {
        var entities = await _repository.GetAllAsync(ct);
        return _mapper.Map<IEnumerable<Item>>(entities);
    }
    
    // Implement other methods...
}"#);

        if include_examples {
            builder = builder
                .heading(2, "Service Registration")
                .code_block("csharp", r#"// Register services in DI
services.AddScoped<IDataService, DataService>();
services.AddScoped<IRepository, Repository>();
services.AddSingleton<IMapper, Mapper>();

// Use in ViewModel
public class MainViewModel
{
    public MainViewModel(IDataService dataService) { }
}"#);
        }

        builder.heading(2, "Best Practices")
            .task_list(vec![(true, "Define interfaces for services"), (true, "Inject dependencies via constructor"), (true, "Use async/await"), (true, "Handle exceptions at service level"), (false, "Implement caching where appropriate")])
            .build()
    }

    fn generate_repository_pattern(&self, _include_examples: bool) -> String {
        MarkdownOutputBuilder::new()
            .heading(1, "Repository Pattern")
            .paragraph("Generic repository pattern for data access abstraction.")
            .heading(2, "Generic Repository")
            .code_block("csharp", r#"public interface IRepository<T> where T : class
{
    Task<IEnumerable<T>> GetAllAsync();
    Task<T?> GetByIdAsync(int id);
    Task<T> AddAsync(T entity);
    Task UpdateAsync(T entity);
    Task DeleteAsync(int id);
}

public class Repository<T> : IRepository<T> where T : class
{
    protected readonly DbContext _context;
    protected readonly DbSet<T> _dbSet;
    
    public Repository(DbContext context)
    {
        _context = context;
        _dbSet = context.Set<T>();
    }
    
    public async Task<IEnumerable<T>> GetAllAsync() =>
        await _dbSet.ToListAsync();
    
    public async Task<T?> GetByIdAsync(int id) =>
        await _dbSet.FindAsync(id);
    
    public async Task<T> AddAsync(T entity)
    {
        await _dbSet.AddAsync(entity);
        await _context.SaveChangesAsync();
        return entity;
    }
    
    // Implement Update and Delete...
}"#)
            .build()
    }

    fn generate_unit_of_work(&self, _include_examples: bool) -> String {
        MarkdownOutputBuilder::new()
            .heading(1, "Unit of Work Pattern")
            .paragraph("Unit of Work for coordinating multiple repositories.")
            .heading(2, "IUnitOfWork Interface")
            .code_block("csharp", r#"public interface IUnitOfWork : IDisposable
{
    IRepository<User> Users { get; }
    IRepository<Order> Orders { get; }
    Task<int> SaveChangesAsync(CancellationToken ct = default);
    Task BeginTransactionAsync();
    Task CommitTransactionAsync();
    Task RollbackTransactionAsync();
}

public class UnitOfWork : IUnitOfWork
{
    private readonly DbContext _context;
    private IRepository<User>? _users;
    private IRepository<Order>? _orders;
    
    public IRepository<User> Users => 
        _users ??= new Repository<User>(_context);
    
    public IRepository<Order> Orders =>
        _orders ??= new Repository<Order>(_context);
    
    public async Task<int> SaveChangesAsync(CancellationToken ct = default) =>
        await _context.SaveChangesAsync(ct);
}"#)
            .build()
    }

    fn generate_mediat_r_pattern(&self, _include_examples: bool) -> String {
        MarkdownOutputBuilder::new()
            .heading(1, "Mediator Pattern with MediatR")
            .paragraph("CQRS and mediator pattern for decoupled architecture.")
            .heading(2, "Commands and Queries")
            .code_block("csharp", r#"// Command
public class CreateUserCommand : IRequest<int>
{
    public string Name { get; set; }
    public string Email { get; set; }
}

public class CreateUserHandler : IRequestHandler<CreateUserCommand, int>
{
    private readonly IRepository<User> _repo;

    public CreateUserHandler(IRepository<User> repo) => _repo = repo;

    public async Task<int> Handle(CreateUserCommand request, CancellationToken ct)
    {
        var user = new User { Name = request.Name, Email = request.Email };
        await _repo.AddAsync(user);
        return user.Id;
    }
}

// Query
public class GetUserQuery : IRequest<User>
{
    public int Id { get; set; }
}

// Usage in ViewModel
public class UserViewModel
{
    private readonly IMediator _mediator;

    public UserViewModel(IMediator mediator) => _mediator = mediator;

    public async Task<int> CreateUserAsync(string name, string email)
    {
        return await _mediator.Send(new CreateUserCommand
        {
            Name = name,
            Email = email
        });
    }
}"#)
            .build()
    }

    #[tool(description = "Creates domain service patterns for complex business logic in AvaloniaUI applications")]
    pub async fn generate_domain_service(&self, params: DomainServiceParams) -> Result<CallToolResult, AvaloniaMcpError> {
        if params.domain_name.is_empty() {
            return Err(AvaloniaMcpError::validation("Domain name cannot be empty"));
        }
        let include_validation = params.include_validation.unwrap_or(true);
        let include_events = params.include_events.unwrap_or(true);
        let validation = if include_validation {
            "\n    public void Validate(Order order)\n    {\n        if (order == null) throw new ArgumentNullException(nameof(order));\n        if (order.TotalAmount < 0) throw new ArgumentException(\"Amount cannot be negative\");\n    }"
        } else { "" };
        let events = if include_events {
            "\n\n    public event EventHandler<OrderCreatedEventArgs> OrderCreated;\n    protected virtual void OnOrderCreated(Order order) =>\n        OrderCreated?.Invoke(this, new OrderCreatedEventArgs(order));"
        } else { "" };
        let d = &params.domain_name;
        let builder = MarkdownOutputBuilder::new()
            .heading(1, &format!("Domain Service: {}", d))
            .heading(2, "Configuration").task_list(vec![(true, format!("Domain: {}", d)), (true, format!("Validation: {}", include_validation)), (true, format!("Events: {}", include_events))])
            .heading(2, "Interface").code_block("csharp", &format!("public interface I{d}Service\n{{\n    Task<TResult> ExecuteAsync<TResult>(Func<Task<TResult>> operation);\n}}"))
            .heading(2, "Implementation").code_block("csharp", &format!("public class {d}Service : I{d}Service\n{{\n    private readonly IRepository _repository;\n    private readonly ILogger _logger;{validation}{events}\n\n    public {d}Service(IRepository repository, ILogger<{d}Service> logger)\n    {{\n        _repository = repository;\n        _logger = logger;\n    }}\n\n    public async Task<TResult> ExecuteAsync<TResult>(Func<Task<TResult>> op)\n    {{\n        _logger.LogInformation(\"Starting {{Domain}} operation\", \"{d}\");\n        try {{\n            var result = await op();\n            return result;\n        }}\n        catch (Exception ex) {{\n            _logger.LogError(ex, \"Error in {{Domain}} operation\", \"{d}\");\n            throw;\n        }}\n    }}\n}}"))
            .heading(2, "DDD Principles").list(&["Aggregate roots control access", "Value objects are immutable", "Domain events capture state changes", "Services orchestrate complex operations"]);
        Ok(CallToolResult::success(vec![Content::text(builder.build())]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_generate_service_layer() {
        let tool = ServiceLayerTool::new();
        let result = tool.generate_service_layer(ServiceLayerParams { service_type: None, include_examples: Some(true) }).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_domain_service_success() {
        let tool = ServiceLayerTool::new();
        let params = DomainServiceParams {
            domain_name: "Order".to_string(),
            include_validation: Some(true),
            include_events: Some(true),
        };
        let result = tool.generate_domain_service(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_domain_service_empty_name() {
        let tool = ServiceLayerTool::new();
        let params = DomainServiceParams {
            domain_name: "".to_string(),
            include_validation: None,
            include_events: None,
        };
        let result = tool.generate_domain_service(params).await;
        assert!(result.is_err());
    }
}
