using System.Text.Json.Serialization;

namespace YDX.Models;

public class UserResponse
{
    [JsonPropertyName("id")]
    public int Id { get; set; }

    [JsonPropertyName("email")]
    public string Email { get; set; } = string.Empty;

    [JsonPropertyName("icon")]
    public string? Icon { get; set; }

    [JsonPropertyName("fname")]
    public string? Fname { get; set; }

    [JsonPropertyName("lname")]
    public string? Lname { get; set; }
}
