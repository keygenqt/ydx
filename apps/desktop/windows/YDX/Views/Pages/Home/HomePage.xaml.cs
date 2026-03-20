using Microsoft.UI.Xaml.Controls;

namespace YDX.Views.Pages.Home;

public sealed partial class HomePage : Page
{
    public HomeViewModel ViewModel { get; } = App.GetService<HomeViewModel>();

    public HomePage()
    {
        InitializeComponent();
    }
}
