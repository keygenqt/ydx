using System;
using System.Text.Json.Serialization;

namespace YDX.Models;

public class TokenResponse
{
    [JsonPropertyName("user_id")]
    public int UserId { get; set; }

    [JsonPropertyName("access_token")]
    public string AccessToken { get; set; } = string.Empty;

    [JsonPropertyName("refresh_token")]
    public string RefreshToken { get; set; } = string.Empty;

    [JsonPropertyName("expires_at")]
    public long ExpiresAt { get; set; }

    [JsonPropertyName("created_at")]
    public long CreatedAt { get; set; } 

    [JsonIgnore]
    public DateTime ExpiresAtDateTime => DateTimeOffset.FromUnixTimeMilliseconds(ExpiresAt).LocalDateTime;

    [JsonIgnore]
    public DateTime CreatedAtDateTime => DateTimeOffset.FromUnixTimeMilliseconds(CreatedAt).LocalDateTime;
}
