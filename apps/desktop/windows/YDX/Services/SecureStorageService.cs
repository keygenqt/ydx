using Redpoint.ThirdParty.CredentialManagement;
using System;
using YDX.Utils;

namespace YDX.Services;

public class SecureStorageService : ISecureStorageService
{
    private const string TargetPrefix = "YDX_";

    private static string GetTarget(string key) => $"{TargetPrefix}{key}";

    public string? GetUserId()
        => GetCredential(AppConstants.SecretKeys.UserId);

    public void SetUserId(string? value)
        => SetCredential(AppConstants.SecretKeys.UserId, value);

    public string? GetAccessToken()
        => GetCredential(AppConstants.SecretKeys.AccessToken);

    public void SetAccessToken(string? value)
        => SetCredential(AppConstants.SecretKeys.AccessToken, value);

    public string? GetRefreshToken()
        => GetCredential(AppConstants.SecretKeys.RefreshToken);

    public void SetRefreshToken(string? value)
        => SetCredential(AppConstants.SecretKeys.RefreshToken, value);

    private static string? GetCredential(string key)
    {
        using var cred = new Credential
        {
            Target = GetTarget(key)
        };
        cred.Load();
        return string.IsNullOrEmpty(cred.Password) ? null : cred.Password;
    }

    private static void SetCredential(string key, string? value)
    {
        using var cred = new Credential
        {
            Target = GetTarget(key),
            Username = Environment.UserName,
            Password = value ?? string.Empty,
            Type = CredentialType.Generic,
            PersistanceType = PersistanceType.LocalComputer
        };

        if (string.IsNullOrEmpty(value))
            cred.Delete();
        else
            cred.Save();
    }

    public void ClearAll()
    {
        SetUserId(null);
        SetAccessToken(null);
        SetRefreshToken(null);
    }
}
