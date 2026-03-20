using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;
using System;
using System.Text.Json;
using System.Threading.Tasks;
using YDX.Models;
using YDX.Native;
using YDX.Services;
using YDX.Strings;
using YDX.Views.Pages.Welcome;

namespace YDX.Views.Pages.Home;

public partial class HomeViewModel(
    INavigationService _navigation,
    ISecureStorageService _secureStorage,
    IDialogService _dialogService) : ObservableObject
{
    [ObservableProperty]
    private string _dataLabel = Resources.FetchData;

    [ObservableProperty]
    private bool _isLoading;

    [RelayCommand]
    private async Task FetchData()
    {
        IsLoading = true;
        try
        {
            var userId = _secureStorage.GetUserId();
            var token = _secureStorage.GetAccessToken();

            if (userId == null || token == null)
            {
                throw new InvalidOperationException(Resources.UserNotFound);
            }

            string jsonResponse = await Task.Run(() => NativeMethods.RequestGetUser(userId, token));

            var response = JsonSerializer.Deserialize<UserResponse>(jsonResponse)
                ?? throw new InvalidOperationException(Resources.FailedParseUserData);

            DataLabel = $"Email: {response.Email}";
        }
        catch (Exception ex)
        {
            DataLabel = Resources.FailedFetchData;
            println($"Error in request: {ex.Message}");
        }
        finally
        {
            IsLoading = false;
        }
    }

    [RelayCommand]
    private async Task ShowAbout()
    {
        await _dialogService.ShowAboutDialogAsync();
    }

    [RelayCommand]
    private async Task Logout()
    {
        var confirm = await _dialogService.ShowConfirmDialogAsync(
            Resources.Logout,
            Resources.LogoutConfirmationMessage);

        if (confirm)
        {
            _secureStorage.ClearAll();
            _navigation.ReplaceRoot<WelcomePage>();
        }
    }
}
