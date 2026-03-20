using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;
using Microsoft.WindowsAppSDK.Runtime.Packages;
using System;
using System.Text.Json;
using System.Text.RegularExpressions;
using System.Threading.Tasks;
using Windows.System;
using YDX.Models;
using YDX.Native;
using YDX.Services;
using YDX.Strings;
using YDX.Utils;
using YDX.Views.Pages.Home;
using YDX.Views.Pages.SignIn;

namespace YDX.Views.Pages.SignUp;

public partial class SignUpViewModel(
    INavigationService _navigation,
    ISecureStorageService _secureStorage) : ObservableObject
{
    [GeneratedRegex(@"^[^\s@]+@[^\s@]+\.[^\s@]+$")]
    private static partial Regex EmailRegex();

    [GeneratedRegex(@"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d).{8,16}$")]
    private static partial Regex PasswordRegex();

    [ObservableProperty]
    private string _email = string.Empty;

    [ObservableProperty]
    private string _password = string.Empty;

    [ObservableProperty]
    private bool _isLoading;

    [ObservableProperty]
    private string _emailError = string.Empty;

    [ObservableProperty]
    private string _passwordError = string.Empty;

    [ObservableProperty]
    private bool _showEmailError;

    [ObservableProperty]
    private bool _showPasswordError;

    private static string IsValidEmail(string email)
    {
        if (string.IsNullOrEmpty(email))
            return Resources.EmailErrorRequired;
        if (!EmailRegex().IsMatch(email))
            return Resources.EmailErrorInvalid;
        return string.Empty;
    }

    private static string IsValidPassword(string password)
    {
        if (string.IsNullOrEmpty(password))
            return Resources.PasswordErrorRequired;
        if (password.Length < 8 || password.Length > 16)
            return Resources.PasswordErrorLength;
        if (!PasswordRegex().IsMatch(password))
            return Resources.PasswordErrorComplexity;
        return string.Empty;
    }

    private bool ValidateFields()
    {
        EmailError = IsValidEmail(Email);
        PasswordError = IsValidPassword(Password);

        ShowEmailError = !string.IsNullOrEmpty(EmailError);
        ShowPasswordError = !string.IsNullOrEmpty(PasswordError);

        return string.IsNullOrEmpty(EmailError) && string.IsNullOrEmpty(PasswordError);
    }

    partial void OnEmailChanged(string value)
    {
        if (!string.IsNullOrEmpty(value))
        {
            EmailError = IsValidEmail(Email);
            ShowEmailError = !string.IsNullOrEmpty(EmailError);
        }
        else
        {
            ShowEmailError = false;
        }
    }

    partial void OnPasswordChanged(string value)
    {
        if (!string.IsNullOrEmpty(value))
        {
            PasswordError = IsValidPassword(Password);
            ShowPasswordError = !string.IsNullOrEmpty(PasswordError);
        }
        else
        {
            ShowPasswordError = false;
        }
    }

    [RelayCommand]
    private async Task SignUp()
    {
        if (!ValidateFields())
            return;

        IsLoading = true;

        try
        {
            string jsonResponse = await Task.Run(() => NativeMethods.RequestSignUp(Email, Password));
            var response = JsonSerializer.Deserialize<ApiResponse>(jsonResponse);

            if (response?.Code == 200 && response.Data.HasValue)
            {
                var tokenData = JsonSerializer.Deserialize<TokenResponse>(response.Data.Value.GetRawText());
                if (tokenData != null)
                {
                    _secureStorage.SetAccessToken(tokenData.AccessToken);
                    _secureStorage.SetRefreshToken(tokenData.RefreshToken);
                    _secureStorage.SetUserId(tokenData.UserId.ToString());
                    _navigation.ReplaceRoot<HomePage>();
                    return;
                }
            }
            else if (response?.Code == 422 && response.Data != null)
            {
                var dataElement = (JsonElement)response.Data;

                EmailError = ErrorHandler.GetError(dataElement, "email");
                ShowEmailError = !string.IsNullOrEmpty(EmailError);

                PasswordError = ErrorHandler.GetError(dataElement, "password");
                ShowPasswordError = !string.IsNullOrEmpty(PasswordError);
            }
            else if (response?.Code == 409)
            {
                EmailError = Resources.EmailExists;
                ShowEmailError = true;
            }
            else
            {
                EmailError = Resources.VerifyEmail;
                ShowEmailError = true;

                PasswordError = Resources.VerifyPassword;
                ShowPasswordError = true;
            }
        }
        catch (Exception)
        {
            EmailError = Resources.ServerEmailError;
            ShowEmailError = true;

            PasswordError = Resources.ServerPasswordError;
            ShowPasswordError = true;
        }
        finally
        {
            IsLoading = false;
        }
    }

    [RelayCommand]
    private void ToSignIn()
    {
        _navigation.Replace<SignInPage>();
    }
}
