using Microsoft.UI.Xaml.Controls;

namespace YDX.Views.Pages.Welcome;

public sealed partial class WelcomePage : Page
{
    public WelcomeViewModel ViewModel { get; } = App.GetService<WelcomeViewModel>();

    public WelcomePage()
    {
        InitializeComponent();
    }
}
