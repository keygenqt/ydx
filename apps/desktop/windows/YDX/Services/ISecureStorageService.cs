namespace YDX.Services;

public interface ISecureStorageService
{
    string? GetUserId();
    void SetUserId(string? value);

    string? GetAccessToken();
    void SetAccessToken(string? value);

    string? GetRefreshToken();
    void SetRefreshToken(string? value);

    void ClearAll();
}
