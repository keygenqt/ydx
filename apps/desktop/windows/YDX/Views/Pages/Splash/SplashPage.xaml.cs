using Microsoft.UI.Xaml.Controls;

namespace YDX.Views.Pages.Splash;

public sealed partial class SplashPage : Page
{
    public SplashViewModel ViewModel { get; } = App.GetService<SplashViewModel>();

    public SplashPage()
    {
        InitializeComponent();

        Loaded += (s, e) => ViewModel.StartSplash();
    }
}
