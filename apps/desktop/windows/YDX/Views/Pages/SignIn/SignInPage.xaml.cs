using Microsoft.UI.Xaml.Controls;

namespace YDX.Views.Pages.SignIn;

public sealed partial class SignInPage : Page
{
    public SignInViewModel ViewModel { get; } = App.GetService<SignInViewModel>();

    public SignInPage()
    {
        InitializeComponent();
    }
}
