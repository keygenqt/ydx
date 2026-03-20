using CommunityToolkit.Mvvm.ComponentModel;
using Microsoft.WindowsAppSDK.Runtime.Packages;
using System;
using System.Collections.Generic;
using System.Text.Json;
using System.Threading.Tasks;
using YDX.Models;
using YDX.Native;
using YDX.Services;
using YDX.Strings;
using YDX.Views.Pages.Home;
using YDX.Views.Pages.Welcome;

namespace YDX.Views.Pages.Splash;
public partial class SplashViewModel(
    INavigationService _navigation,
    ISecureStorageService _secureStorage) : ObservableObject
{
    private readonly List<string> _messages =
    [
        Resources.SplashBrewingCoffee,
        Resources.SplashSteepingTea,
        Resources.SplashTicklingDatabase,
        Resources.SplashIgnitingReactor,
        Resources.SplashInspectingTokens,
        Resources.SplashCastingSpells,
        Resources.SplashPullingRabbit,
        Resources.SplashFeedingUnicorns,
        Resources.SplashAcceleratingNeutrons,
        Resources.SplashMixingReagents,
        Resources.SplashWateringServer
    ];

    [ObservableProperty]
    private string _currentMessage = string.Empty;

    private readonly Random _random = new();
    private int _currentMessageIndex = -1;

    public void StartSplash()
    {
        _ = RotateMessagesAsync();
        _ = CheckAuthAsync();
    }

    private async Task RotateMessagesAsync()
    {
        ShowRandomMessage();
        while (_currentMessageIndex >= 0)
        {
            await Task.Delay(2000);
            ShowRandomMessage();
        }
    }

    private void ShowRandomMessage()
    {
        int newIndex;
        do
        {
            newIndex = _random.Next(_messages.Count);
        }
        while (newIndex == _currentMessageIndex && _messages.Count > 1);

        _currentMessageIndex = newIndex;
        CurrentMessage = _messages[_currentMessageIndex];
    }

    private async Task CheckAuthAsync()
    {
        var token = _secureStorage.GetAccessToken();

        if (!string.IsNullOrEmpty(token))
        {
            await Task.Delay(800);
            try
            {
                string jsonResponse = await Task.Run(() => NativeMethods.RequestSignCheck(token));
                var response = JsonSerializer.Deserialize<ApiResponse>(jsonResponse);
                if(response?.Code == 200)
                {
                    _navigation.ReplaceRoot<HomePage>();
                    return;
                }
            }
            catch (Exception ex)
            {
                println($"Error in request: {ex.Message}");
            }

            _navigation.ReplaceRoot<WelcomePage>();
        }
        else
        {
            _navigation.ReplaceRoot<WelcomePage>(false);
        }
    }
}
