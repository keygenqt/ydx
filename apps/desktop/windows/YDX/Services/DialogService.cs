using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using System;
using System.Threading.Tasks;
using YDX.Views.Dialogs;

namespace YDX.Services;

public class DialogService : IDialogService
{
    private Window? _window;

    public void Initialize(Window window)
    {
        _window = window;
    }

    private void EnsureInitialized()
    {
        if (_window == null)
            throw new InvalidOperationException("DialogService not initialized. Call Initialize first.");
    }

    public async Task ShowAboutDialogAsync()
    {
        EnsureInitialized();

        var dialog = new AboutDialog()
        {
            XamlRoot = _window!.Content.XamlRoot
        };

        var result = await dialog.ShowAsync();

        if (result == ContentDialogResult.Primary)
        {
            await Windows.System.Launcher.LaunchUriAsync(new Uri("https://dev.ydx.keygenqt.com"));
        }
    }

    public async Task<bool> ShowConfirmDialogAsync(string title, string message)
    {
        EnsureInitialized();

        var dialog = new ConfirmDialog(title, message)
        {
            XamlRoot = _window!.Content.XamlRoot
        };

        return await dialog.ShowAsync() == ContentDialogResult.Primary;
    }
}
