using System.Text.Json;

namespace YDX.Utils;

public static class ErrorHandler
{
    public static string GetError(JsonElement data, string fieldName)
    {
        if (data.TryGetProperty("errors", out var errors) &&
            errors.TryGetProperty(fieldName, out var fieldErrors) &&
            fieldErrors.GetArrayLength() > 0 &&
            fieldErrors[0].TryGetProperty("message", out var message))
        {
            return message.GetString() ?? string.Empty;
        }

        return string.Empty;
    }
}
