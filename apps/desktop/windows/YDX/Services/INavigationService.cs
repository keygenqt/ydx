using Microsoft.UI.Xaml.Controls;
using System;

namespace YDX.Services;

public interface INavigationService
{
    void Initialize(Frame frame);
    void NavigateTo<T>() where T : Page;
    void Replace<T>() where T : Page;
    void ReplaceRoot<T>(bool animate = true) where T : Page;
    void GoBack();
    bool CanGoBack();

    event EventHandler? CanGoBackChanged;
}
