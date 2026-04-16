//! API Integration tool - REST API client generation
use avalonia_mcp_core::error::AvaloniaMcpError;
use avalonia_mcp_core::markdown::MarkdownOutputBuilder;
use rmcp::model::{CallToolResult, Content};
use rmcp::tool;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct APIIntegrationParams {
    pub api_type: Option<String>,
    pub include_examples: Option<bool>,
}

/// API models generation parameters
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ApiModelsParams {
    /// Entity name (e.g., User, Product, Order)
    pub entity_name: String,
    /// Model type: 'request', 'response', 'entity', 'dto'
    pub model_type: Option<String>,
    /// Include validation attributes
    pub include_validation: Option<bool>,
}

#[derive(Debug, Clone, Default)]
pub struct APIIntegrationTool;

impl APIIntegrationTool {
    pub fn new() -> Self { Self }

    #[tool(description = "Generate REST API client integration patterns for AvaloniaUI applications. Covers HttpClient, Refit, Polly retry policies, and error handling.")]
    pub async fn generate_api_integration(
        &self,
        params: APIIntegrationParams,
    ) -> Result<CallToolResult, AvaloniaMcpError> {
        let include_examples = params.include_examples.unwrap_or(true);
        let api_type = params.api_type.as_deref().unwrap_or("rest");

        let output = match api_type {
            "rest" => self.generate_rest_api(include_examples),
            "graphql" => self.generate_graphql_api(include_examples),
            "grpc" => self.generate_grpc_api(include_examples),
            _ => self.generate_rest_api(include_examples),
        };

        Ok(CallToolResult::success(vec![Content::text(output)]))
    }

    fn generate_rest_api(&self, include_examples: bool) -> String {
        let mut builder = MarkdownOutputBuilder::new()
            .heading(1, "REST API Integration")
            .paragraph("Best practices for integrating REST APIs in AvaloniaUI applications.")
            .heading(2, "Key Components")
            .list(vec!["HttpClient factory", "Refit interface", "Polly policies", "Error handling"]);

        if include_examples {
            builder = builder
                .heading(2, "Refit API Client")
                .code_block("csharp", r#"// Define API interface
public interface IApiClient
{
    [Get("/api/users")]
    Task<List<User>> GetUsersAsync();
    
    [Post("/api/users")]
    Task<User> CreateUserAsync([Body] User user);
    
    [Get("/api/users/{id}")]
    Task<User> GetUserAsync(int id);
    
    [Delete("/api/users/{id}")]
    Task DeleteUserAsync(int id);
}

// Configure in DI
services.AddRefitClient<IApiClient>()
    .ConfigureHttpClient(c => c.BaseAddress = new Uri("https://api.example.com"))
    .AddPolicyHandler(GetRetryPolicy())
    .AddPolicyHandler(GetCircuitBreakerPolicy());

// Policies
static IAsyncPolicy<HttpResponseMessage> GetRetryPolicy() =>
    HttpPolicyExtensions.HandleTransientHttpErrors()
        .WaitAndRetryAsync(3, retryAttempt => 
            TimeSpan.FromSeconds(Math.Pow(2, retryAttempt)));

static IAsyncPolicy<HttpResponseMessage> GetCircuitBreakerPolicy() =>
    HttpPolicyExtensions.HandleTransientHttpErrors()
        .CircuitBreakerAsync(5, TimeSpan.FromSeconds(30));"#)
                .heading(2, "ViewModel Integration")
                .code_block("csharp", r#"public class UsersViewModel : ViewModelBase
{
    private readonly IApiClient _apiClient;
    private ObservableCollection<User> _users;
    
    public UsersViewModel(IApiClient apiClient)
    {
        _apiClient = apiClient;
        _ = LoadUsersAsync();
    }
    
    private async Task LoadUsersAsync()
    {
        try
        {
            IsLoading = true;
            Users = await _apiClient.GetUsersAsync();
        }
        catch (ApiException ex) when (ex.StatusCode == HttpStatusCode.NotFound)
        {
            // Handle 404
        }
        catch (Exception ex)
        {
            // Handle other errors
        }
        finally
        {
            IsLoading = false;
        }
    }
}"#);
        }

        builder.heading(2, "Best Practices")
            .task_list(vec![(true, "Use dependency injection"), (true, "Implement retry policies"), (true, "Handle errors gracefully"), (true, "Cancel requests on navigation"), (false, "Add request caching")])
            .build()
    }

    fn generate_graphql_api(&self, _include_examples: bool) -> String {
        MarkdownOutputBuilder::new()
            .heading(1, "GraphQL Integration")
            .paragraph("GraphQL client integration using GraphQL.Client")
            .heading(2, "Setup")
            .code_block("csharp", r#"// Install: dotnet add package GraphQL.Client
// Install: dotnet add package GraphQL.Client.Serializer.Newtonsoft

var graphQlClient = new GraphQLClient("https://api.example.com/graphql");
var request = new GraphQLRequest { Query = "{ users { id name } }" };
var response = await graphQlClient.SendQueryAsync<UserResult>(request);"#)
            .build()
    }

    fn generate_grpc_api(&self, _include_examples: bool) -> String {
        MarkdownOutputBuilder::new()
            .heading(1, "gRPC Integration")
            .paragraph("gRPC client integration for high-performance APIs")
            .heading(2, "Setup")
            .code_block("csharp", r#"// Install: dotnet add package Grpc.Net.Client
// Add .proto file and generate client code

var channel = GrpcChannel.ForAddress("https://api.example.com");
var client = new UserService.UserServiceClient(channel);
var response = await client.GetUserAsync(new GetUserRequest { Id = 1 });"#)
            .build()
    }

    #[tool(description = "Creates data transfer objects (DTOs) and model classes for API integration in AvaloniaUI applications")]
    pub async fn generate_api_models(
        &self,
        params: ApiModelsParams,
    ) -> Result<CallToolResult, AvaloniaMcpError> {
        if params.entity_name.is_empty() {
            return Err(AvaloniaMcpError::validation("Entity name cannot be empty"));
        }

        let model_type = params.model_type.as_deref().unwrap_or("dto").to_lowercase();
        let include_validation = params.include_validation.unwrap_or(true);
        let entity_lower = params.entity_name.to_lowercase();
        let model_type_cap = match model_type.as_str() {
            "request" => "Request",
            "response" => "Response",
            "entity" => "Entity",
            _ => "Dto",
        };

        let validation_section = if include_validation {
            r#"
    [Required]
    [StringLength(100)]
    public string Name { get; set; } = string.Empty;

    [EmailAddress]
    public string? Email { get; set; }"#
        } else {
            r#"
    public string Name { get; set; } = string.Empty;
    public string? Email { get; set; }"#
        };

        let model_code = format!(
            r#"// {model_type} model for {entity}
public class {entity}{mt}
{{{{
    public int Id {{{{ get; set; }}}}
{va}

    public DateTime CreatedAt {{{{ get; set; }}}} = DateTime.UtcNow;
    public bool IsActive {{{{ get; set; }}}} = true;
}}}}

// Request model
public class Create{entity}Request
{{{{
{va}
}}}}

// Response model
public class {entity}Response
{{{{
    public bool IsSuccess {{{{ get; set; }}}}
    public {entity}{mt}? Data {{{{ get; set; }}}}
    public string ErrorMessage {{{{ get; set; }}}} = string.Empty;
}}}}"#,
            entity = params.entity_name,
            mt = model_type_cap,
            va = validation_section,
        );

        let usage = format!(
            r#"// Creating a request
var request = new Create{entity}Request
{{{{
    Name = "Example"
}}}};

// Sending request
var response = await _apiClient.PostAsync<{entity}{mt}>("api/{lower}", request);

// Handling response
if (response.IsSuccess)
{{{{
    var {lower} = response.Data;
    // Process successful response
}}}}
else
{{{{
    Console.WriteLine(response.ErrorMessage);
}}}}"#,
            entity = params.entity_name,
            mt = model_type_cap,
            lower = entity_lower,
        );

        let builder = MarkdownOutputBuilder::new()
            .heading(1, &format!("API Models: {}", params.entity_name))
            .heading(2, "Configuration")
            .task_list(vec![
                (true, format!("Entity: {}", params.entity_name)),
                (true, format!("Model Type: {}", model_type)),
                (true, format!("Validation: {}", include_validation)),
            ])
            .heading(2, "Model Classes")
            .code_block("csharp", &model_code)
            .heading(2, "Usage Example")
            .code_block("csharp", &usage)
            .heading(2, "Serialization Configuration")
            .code_block(
                "csharp",
                r#"var jsonOptions = new JsonSerializerOptions {
    PropertyNamingPolicy = JsonNamingPolicy.CamelCase,
    DefaultIgnoreCondition = JsonIgnoreCondition.WhenWritingNull,
    Converters = { new JsonStringEnumConverter() }
};"#,
            );

        Ok(CallToolResult::success(vec![Content::text(builder.build())]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_generate_api_integration() {
        let tool = APIIntegrationTool::new();
        let result = tool.generate_api_integration(APIIntegrationParams { api_type: None, include_examples: Some(true) }).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_api_models() {
        let tool = APIIntegrationTool::new();
        let result = tool.generate_api_models(ApiModelsParams {
            entity_name: "User".to_string(),
            model_type: Some("dto".to_string()),
            include_validation: Some(true),
        }).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
    }

    #[tokio::test]
    async fn test_generate_api_models_empty_entity() {
        let tool = APIIntegrationTool::new();
        let result = tool.generate_api_models(ApiModelsParams {
            entity_name: "".to_string(),
            model_type: None,
            include_validation: None,
        }).await;
        assert!(result.is_err());
    }
}
